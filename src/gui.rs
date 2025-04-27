use iced::event::Event;
use iced::keyboard::key;
use iced::widget::scrollable::AbsoluteOffset;
use iced::{keyboard, window, Length, Theme};

use crate::components;
use crate::components::styles;
use crate::app::{App, Message};
use crate::app::Message::Server;
use crate::server;
use crate::server::ServerMessage;
use crate::settings::Settings;
use iced::widget::{self, button, column, container, horizontal_space, row, svg, text};
use iced::{Bottom, Element, Fill, Font, Subscription, Task};

/// Initializes and runs the GUI application
pub fn gui() -> iced::Result {
    let settings = Settings::load();

    iced::application(App::default, App::update, App::view)
        .title("dbug desktop")
        .subscription(App::subscription)
        .font(include_bytes!("../assets/fonts/firacode.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .theme(App::theme)
        .window(window::Settings {
            size: settings.get_window_size(),
            position: window::Position::Specific(settings.get_window_position()),
            ..window::Settings::default()
        })
        .run()
}

impl App {
    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            Subscription::run(server::listen).map(Server),
            iced::event::listen_with(|event, _status, _window_id| match event {
                Event::Window(window::Event::Closed) => Some(Message::WindowClosed),
                Event::Window(window::Event::Moved(position)) => {
                    Some(Message::WindowMoved(position))
                }
                Event::Window(window::Event::Resized(size)) => Some(Message::WindowResized(size)),
                _ => None,
            }),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Server(server_message) => {
                match server_message {
                    ServerMessage::PayloadReceived(value) => {
                        let scroll_command;
                        if let Err(e) = self.storage.add_json(&value) {
                            eprintln!("Failed to store payload: {e}");
                            scroll_command = Task::none();
                        } else {
                            self.payload_list_cache = self.storage.get_all();
                            self.expanded_payload_id = self.payload_list_cache.first().map(|(id, _)| id.clone());
                            self.collapsed_json_lines.clear();

                            scroll_command = widget::scrollable::scroll_to::<Message>(
                                widget::scrollable::Id::new("payload_scroll"),
                                AbsoluteOffset { x: 0.0, y: 0.0 },
                            );
                        }
                        scroll_command
                    }
                }
            }
            Message::ShowModal => {
                self.show_modal = true;
                Task::none()
            }
            Message::HideModal => {
                self.hide_modal();
                Task::none()
            }
            Message::ThemeChanged(index) => {
                if let Some(theme) = Theme::ALL.get(index).cloned() {
                    self.settings.set_theme(theme);
                    if let Err(e) = self.settings.save() {
                        eprintln!("Failed to save settings: {e}");
                    }
                }
                Task::none()
            }
            Message::TogglePayload(id) => {
                if self.expanded_payload_id.as_ref() == Some(&id) {
                    self.expanded_payload_id = None;
                } else {
                    if let Some(_old_id) = self.expanded_payload_id.take() {
                         // No cache to update
                         // self.update_highlight_cache(&old_id);
                    }
                    self.expanded_payload_id = Some(id.clone());
                    // self.update_highlight_cache(&id);
                }
                Task::none()
            }
            Message::ToggleJsonSection(line_index) => {
                 if let Some(_payload_id) = self.expanded_payload_id.clone() {
                    if self.collapsed_json_lines.contains(&line_index) {
                        self.collapsed_json_lines.remove(&line_index);
                    } else {
                        self.collapsed_json_lines.insert(line_index);
                    }
                    // No cache to update
                    // self.update_highlight_cache(&payload_id);
                 } else {
                     eprintln!("WARN: ToggleJsonSection called with no expanded payload");
                 }
                Task::none()
            }
            Message::ClearPayloads => {
                if let Err(e) = self.storage.delete_all() {
                    eprintln!("Failed to clear payloads: {e}");
                } else {
                    self.payload_list_cache.clear();
                    self.expanded_payload_id = None;
                    self.collapsed_json_lines.clear();
                }
                Task::none()
            }
            Message::DeletePayload(id) => {
                let deleted = match self.storage.delete(&id) {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("Failed to delete payload: {e}");
                        false
                    }
                };

                if deleted {
                    self.payload_list_cache.retain(|(item_id, _)| item_id != &id);
                    if self.expanded_payload_id.as_ref() == Some(&id) {
                        self.expanded_payload_id = None;
                    }
                    self.collapsed_json_lines.clear();
                }
                Task::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Escape),
                    ..
                }) => {
                    self.hide_modal();
                    Task::none()
                }
                _ => Task::none(),
            },
            Message::WindowMoved(position) => {
                self.settings.set_window_position(position);
                // Save immediately on move
                if let Err(e) = self.settings.save() {
                    eprintln!("ERROR: Failed to save settings on move: {e}");
                }
                Task::none()
            }
            Message::WindowResized(size) => {
                self.settings.set_window_size(size);
                 // Save immediately on resize
                if let Err(e) = self.settings.save() {
                    eprintln!("ERROR: Failed to save settings on resize: {e}");
                }
                Task::none()
            }
            Message::WindowClosed => {
                // Attempt a final save on close, but don't rely on it solely.
                if let Err(e) = self.settings.save() {
                    // Log quietly if needed, but main saves happen earlier.
                    eprintln!("Note: Final settings save on close failed: {e}");
                }
                iced::exit()
            }
        }
    }

    /// Returns the current theme
    fn theme(&self) -> Theme {
        self.settings.theme()
    }

    /// Renders the application view
    fn view(&self) -> Element<Message> {
        let logo_svg = svg(svg::Handle::from_memory(
            include_bytes!("../assets/icons/mdi--ladybug.svg").as_slice(),
        ))
        .style(styles::svg_style_primary)
        .width(Fill)
        .height(Fill);

        let settings_svg = svg(svg::Handle::from_memory(
            include_bytes!("../assets/icons/mdi--mixer-settings.svg").as_slice(),
        ))
        .style(styles::svg_style_secondary)
        .width(Fill)
        .height(Fill);

        let remove_all_svg = svg(svg::Handle::from_memory(
            include_bytes!("../assets/icons/mdi--delete-variant.svg").as_slice(),
        ))
        .style(styles::svg_style_secondary)
        .width(Fill)
        .height(Fill);

        let button_size = 25;
        let payload_count = self.payload_list_cache.len();

        // Calculate max height based on window size BEFORE the macro call
        let window_size = self.settings.get_window_size();
        let max_payload_height = window_size.height - 100.0;

        let content = container(
            column![
                row![
                    button(logo_svg)
                        .width(button_size)
                        .height(button_size)
                        .padding(3.0),
                    horizontal_space(),
                    text(format!("{payload_count}"))
                        .size(14),
                    button(remove_all_svg)
                        .style(button::danger)
                        .width(button_size)
                        .height(button_size)
                        .padding(3.0)
                        .on_press(Message::ClearPayloads),
                    button(settings_svg)
                        .style(button::secondary)
                        .width(button_size)
                        .height(button_size)
                        .padding(3.0)
                        .on_press(Message::ShowModal),
                ]
                .padding(10)
                .spacing(10)
                .align_y(iced::alignment::Vertical::Center)
                .height(Length::Shrink),

                components::payload_list(
                    &self.payload_list_cache,
                    self.expanded_payload_id.as_ref(),
                    &self.theme(),
                    &self.collapsed_json_lines,
                    max_payload_height, // Pass calculated max height
                ),
                row![horizontal_space()]
                    .align_y(Bottom)
                    .height(Length::Shrink),
            ]
            .height(Fill),
        );

        if self.show_modal {
            let current_theme = self.theme();
            let settings_content = components::settings_modal(current_theme);

            components::modal(content, settings_content, Message::HideModal)
        } else {
            content.into()
        }
    }

    /// Hides the modal dialog
    fn hide_modal(&mut self) {
        self.show_modal = false;
    }
}
