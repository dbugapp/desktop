use iced::event::Event;
use iced::keyboard::key;
use iced::widget::scrollable::AbsoluteOffset;
use iced::{keyboard, window, Length, Theme};

use crate::components;
use crate::components::styles;
use crate::gui::Message::Server;
use crate::server;
use crate::server::ServerMessage;
use crate::settings::Settings;
use crate::storage::Storage;
use iced::widget::{self, button, column, container, horizontal_space, row, svg};
use iced::{Bottom, Element, Fill, Font, Subscription, Task};

/// Initializes and runs the GUI application
pub fn gui() -> iced::Result {
    let settings = Settings::load();

    iced::application(App::default, App::update, App::view).title("dbug desktop")
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

/// Application state and logic
struct App {
    show_modal: bool,
    settings: Settings,
    storage: Storage,
    expanded_payload_id: Option<String>, // Track which payload is currently expanded
}

impl Default for App {
    fn default() -> Self {
        let storage = Storage::new().expect("Failed to initialize storage");
        let newest_payload_id = storage.get_all().first().map(|(id, _)| id.clone());

        Self {
            show_modal: false,
            settings: Settings::load(),
            storage,
            expanded_payload_id: newest_payload_id,
        }
    }
}

/// Messages used for application state updates
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) enum Message {
    ShowModal,
    HideModal,
    Event(Event),
    Server(ServerMessage),
    ThemeChanged(usize),
    TogglePayload(String), // Toggle expansion of a payload by its ID
    ClearPayloads,         // Clear all payloads
    DeletePayload(String), // Delete a payload by its ID
    WindowMoved(iced::Point),
    WindowResized(iced::Size),
    WindowClosed,
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
                println!("{:?}", server_message);
                match server_message {
                    ServerMessage::PayloadReceived(value) => {
                        if let Err(e) = self.storage.add_json(&value) {
                            eprintln!("Failed to store payload: {}", e);
                        }
                        // Immediately expand the newly added payload
                        self.expanded_payload_id =
                            self.storage.get_all().first().map(|(id, _)| id.clone());

                        // Scroll to top to ensure new payload is visible
                        widget::scrollable::scroll_to(
                            widget::scrollable::Id::new("payload_scroll"),
                            AbsoluteOffset { x: 0.0, y: 0.0 },
                        )
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
                        eprintln!("Failed to save settings: {}", e);
                    }
                }
                Task::none()
            }
            Message::TogglePayload(id) => {
                // If this is the currently expanded payload, collapse it
                if self.expanded_payload_id.as_ref() == Some(&id) {
                    self.expanded_payload_id = None;
                } else {
                    // Otherwise, expand this payload and collapse any other
                    self.expanded_payload_id = Some(id);
                }
                Task::none()
            }
            Message::ClearPayloads => {
                if let Err(e) = self.storage.delete_all() {
                    eprintln!("Failed to clear payloads: {}", e);
                }
                Task::none()
            }
            Message::DeletePayload(id) => {
                if let Err(e) = self.storage._delete(&id) {
                    eprintln!("Failed to delete payload: {}", e);
                }
                if self.expanded_payload_id.as_ref() == Some(&id) {
                    self.expanded_payload_id = None;
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
                let _ = self.settings.save();
                Task::none()
            }
            Message::WindowResized(size) => {
                self.settings.set_window_size(size);
                let _ = self.settings.save();
                Task::none()
            }
            Message::WindowClosed => {
                let _ = self.settings.save();
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

        let logo_svg = svg(svg::Handle::from_path("assets/icons/mdi--ladybug.svg"))
            .style(styles::svg_style_primary)
            .width(Fill)
            .height(Fill);

        let settings_svg = svg(svg::Handle::from_path(
            "assets/icons/mdi--mixer-settings.svg",
        ))
        .style(styles::svg_style_secondary)
        .width(Fill)
        .height(Fill);

        let remove_all_svg = svg(svg::Handle::from_path(
            "assets/icons/mdi--trash-can.svg",
        ))
        .style(styles::svg_style_secondary)
        .width(Fill)
        .height(Fill);

        let button_size = 25;

        let content = container(
            column![
                row![
                    button(logo_svg)
                        .width(button_size)
                        .height(button_size)
                        .padding(3.0),
                    horizontal_space(),
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
                .height(Length::Shrink),
                components::payload_list(
                    &self.storage,
                    self.expanded_payload_id.as_ref(),
                    &self.theme()
                ),
                row![horizontal_space()]
                    .align_y(Bottom)
                    .height(Length::Shrink),
            ]
            .height(Fill),
        );

        if self.show_modal {
            // Get the current theme
            let current_theme = self.theme();

            // Use the settings modal component
            let settings_content = components::settings_modal(current_theme);

            components::modal(content, settings_content, Message::HideModal)
        } else {
            content.into()
        }
    }
}

impl App {
    /// Hides the modal dialog
    fn hide_modal(&mut self) {
        self.show_modal = false;
    }
}
