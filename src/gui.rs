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

use global_hotkey::GlobalHotKeyEvent;
use iced::futures::SinkExt;
use iced::stream;
use crate::app::HotkeyAction;

const APP_TITLE: &str = concat!("dbug desktop v", env!("CARGO_PKG_VERSION"));

pub fn gui() -> iced::Result {
    let settings = Settings::load();

    iced::application(App::default, App::update, App::view)
        .title(APP_TITLE)
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

fn hotkey_listener() -> impl futures::Stream<Item = Message> {
    stream::channel(32, |mut sender: futures::channel::mpsc::Sender<Message>| async move {
        let receiver = GlobalHotKeyEvent::receiver();
        loop {
            if let Ok(event) = receiver.try_recv() {
                if sender.send(Message::HotkeyActivated(event.id)).await.is_err() {
                    break;
                }
            }
            async_std::task::sleep(std::time::Duration::from_millis(50)).await;
        }
    })
}

impl App {
    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            Subscription::run(server::listen).map(Server),
            Subscription::run(hotkey_listener),
            iced::event::listen_with(|event, _status, window_id| {
                match event {
                    // Forward Keyboard events
                    Event::Keyboard(_) => Some(Message::Event(event)),
                    // Keep existing Window event handlers
                    Event::Window(window::Event::Closed) => Some(Message::WindowClosed),
                    Event::Window(window::Event::Moved(position)) => Some(Message::WindowMoved(position)),
                    Event::Window(window::Event::Resized(size)) => Some(Message::WindowResized(size)),
                    Event::Window(_) => Some(Message::CaptureWindowId(window_id)),
                    _ => None,
                }
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
                    }
                    self.expanded_payload_id = Some(id.clone());
                    self.search_query.clear();
                    self.collapsed_json_lines.clear();
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
                    self.search_query.clear(); // Clear search when all payloads are cleared
                }
                Task::none()
            }
            Message::DeletePayload(id) => {
                let deleted = self.storage.delete(&id).unwrap_or_else(|e| {
                    eprintln!("Failed to delete payload: {e}");
                    false
                });

                if deleted {
                    self.payload_list_cache.retain(|(item_id, _)| item_id != &id);
                    if self.expanded_payload_id.as_ref() == Some(&id) {
                        self.expanded_payload_id = None;
                        self.search_query.clear(); 
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
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key,
                    modifiers,
                    ..
                }) if modifiers.command() && key == keyboard::Key::Character(",".into()) => {
                    Task::perform(async {}, |()| Message::OpenSettings)
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
                if let Err(e) = self.settings.save() {
                    eprintln!("Note: Final settings save on close failed: {e}");
                }
                iced::exit()
            }
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
                Task::none()
            }
            Message::HotkeyActivated(id) => {
                if let Some(action) = self.hotkey_actions.get(&id) {
                    match action {
                        HotkeyAction::ShowWindow => {
                            window::gain_focus(self.main_window_id.unwrap())
                        }
                        HotkeyAction::ClearPayloads => {
                            Task::perform(async {}, |()| Message::ClearPayloads)
                        }
                    }
                } else {
                    eprintln!("WARN: HotkeyActivated received for unknown ID: {id}");
                    Task::none()
                }
            }
            Message::CaptureWindowId(id) => {
                if self.main_window_id.is_none() {
                    self.main_window_id = Some(id);
                }
                Task::none()
            }
            Message::OpenSettings => {
                self.show_modal = true;
                Task::none()
            }
            Message::ServerHostChanged(host) => {
                self.settings.set_server_host(host);
                if let Err(e) = self.settings.save() {
                    eprintln!("Failed to save settings: {e}");
                }
                Task::none()
            }
            Message::ServerPortChanged(port_str) => {
                if let Ok(port) = port_str.parse::<u16>() {
                    self.settings.set_server_port(port);
                    if let Err(e) = self.settings.save() {
                        eprintln!("Failed to save settings: {e}");
                    }
                }
                Task::none()
            }
            Message::ResetServerToDefaults => {
                self.settings.set_server_host("127.0.0.1".to_string());
                self.settings.set_server_port(53821);
                if let Err(e) = self.settings.save() {
                    eprintln!("Failed to save settings: {e}");
                }
                Task::none()
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
                    max_payload_height,
                    &self.search_query,
                ),
                row![horizontal_space()]
                    .align_y(Bottom)
                    .height(Length::Shrink),
            ]
            .height(Fill),
        );

        if self.show_modal {
            let current_theme = self.theme();
            let settings_content = components::settings_modal(current_theme, &self.settings);

            components::modal(content, settings_content, Message::HideModal)
        } else {
            content.into()
        }
    }

    fn hide_modal(&mut self) {
        self.show_modal = false;
    }
}
