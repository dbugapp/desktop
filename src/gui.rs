use iced::event::{self, Event};
use iced::keyboard;
use iced::futures::channel::mpsc;
use iced::keyboard::key;
use iced::widget::{
    self, button, center, column, container, horizontal_space, mouse_area,
    opaque, row, stack, text, svg,
};
use iced::{Bottom, Color, Element, Fill, Subscription, Task};
use serde_json::Value;
use crate::settings::{Settings, Theme};
use crate::storage::Storage;
use crate::server;
use crate::server::ServerInput;

/// Initializes and runs the GUI application
pub fn gui() -> iced::Result {
    iced::application("Modal - Iced", App::update, App::view)
        .subscription(App::subscription).run()
}

/// Application state and logic
struct App {
    show_modal: bool,
    settings: Settings,
    storage: Storage,
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_modal: false,
            settings: Settings::load(),
            storage: Storage::new().expect("Failed to initialize storage"),
        }
    }
}

/// Messages used for application state updates
#[derive(Debug, Clone)]
pub(crate) enum Message {
    ShowModal,
    HideModal,
    ThemeSelected(Theme),
    Event(Event),
    Server(ServerMessage), // Renamed for clarity

}

#[derive(Debug, Clone)]
pub enum ServerMessage { // The server event types
    Ready(mpsc::Sender<ServerInput>),
    PayloadReceived(Value),
    WorkFinished,

}


impl App {

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(server::listen)
    }


    /// Updates application state based on messages
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Server(ServerMessage) => {
                Task::none()
            }
            Message::ShowModal => {
                self.show_modal = true;
                Task::none()
            }
            Message::HideModal => {
                self.hide_modal();
                Task::none()
            }
            Message::ThemeSelected(theme) => {
                self.settings.theme = theme;
                if let Err(e) = self.settings.save() {
                    eprintln!("Failed to save settings: {}", e);
                }
                self.hide_modal();
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
        }
    }

    /// Renders the application view
    fn view(&self) -> Element<Message> {
        let handle = svg::Handle::from_path("src/assets/icons/mdi--mixer-settings.svg");
        let content = container(
            column![
                row![
                    horizontal_space(),
                    button(svg(handle).width(20).height(20)).on_press(Message::ShowModal)
                ]
                .height(Fill),
                column(
                    self.storage.get_all().iter().map(|(_, value)| {
                        container(
                            row![
                                text(format!("{}", value))
                            ]
                            .spacing(10)
                        )
                        .style(container::rounded_box)
                        .padding(10)
                        .into()
                    }).collect::<Vec<_>>()
                )
                .spacing(10)
                .padding(10),
                row![
                    horizontal_space()
                ]
                .align_y(Bottom)
                .height(Fill),
            ]
                .height(Fill),
        )
            .padding(10);

        if self.show_modal {
            let theme_selection = container(
                column![
                    text("Select Theme").size(24),
                    row![
                        button(text("Dark")).on_press(Message::ThemeSelected(Theme::Dark)),
                        button(text("Light")).on_press(Message::ThemeSelected(Theme::Light)),
                        button(text("System")).on_press(Message::ThemeSelected(Theme::System)),
                    ]
                    .spacing(10)
                ]
                    .spacing(20),
            )
                .width(300)
                .padding(10)
                .style(container::rounded_box);

            modal(content, theme_selection, Message::HideModal)
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

/// Creates a modal dialog overlay
fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
        .into()
}