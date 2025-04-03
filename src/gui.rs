use iced::event::Event;
use iced::keyboard::key;
use iced::{keyboard, Length};

use crate::gui::Message::Server;
use crate::server;
use crate::server::ServerMessage;
use crate::settings::{Appearance, Settings};
use crate::storage::Storage;
use crate::theme::{Mode, Theme};
use iced::widget::{
    self, button, center, column, container, horizontal_space, mouse_area, opaque, row, scrollable,
    stack, svg, text,
};
use iced::{Bottom, Color, Element, Fill, Subscription, Task};

/// Initializes and runs the GUI application
pub fn gui() -> iced::Result {
    iced::application("Modal - Iced", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

/// Application state and logic
struct App {
    show_modal: bool,
    settings: Settings,
    storage: Storage,
    theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_modal: false,
            settings: Settings::load(),
            storage: Storage::new().expect("Failed to initialize storage"),
            theme: Theme::default(),
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
    AppearanceSelected(Appearance),
}

impl App {
    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(server::listen).map(Server)
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
                    }
                }
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
            Message::AppearanceSelected(appearance) => {
                self.settings.appearance = appearance;
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
        let storage_rows = column(
            self.storage
                .get_all()
                .iter()
                .map(|(_, value)| {
                    container(row![text(format!("{}", value))].spacing(10))
                        .style(container::rounded_box)
                        .padding(10)
                        .width(Fill)
                        .into()
                })
                .collect::<Vec<_>>(),
        )
        .spacing(10)
        .padding(10);
        let scrollable_storage = scrollable(storage_rows).width(Fill).height(Fill);
        let content = container(
            column![
                row![
                    horizontal_space(),
                    button(svg(handle).width(20).height(20))
                        .on_press(Message::ShowModal)
                        .style(|_theme, _state| button::Style {
                            background: Some(self.theme.primary.into()),
                            ..button::Style::default()
                        })
                ]
                .height(Length::Shrink),
                scrollable_storage,
                row![horizontal_space()]
                    .align_y(Bottom)
                    .height(Length::Shrink),
            ]
            .height(Fill),
        )
        .padding(10)
        .style(|_theme| container::Style {
            background: Some(self.theme.bg.into()),
            ..container::Style::default()
        });

        if self.show_modal {
            let theme_selection = container(
                column![
                    text("Select Theme").size(24),
                    row![
                        button(text("Dark"))
                            .on_press(Message::AppearanceSelected(Appearance::Dark)),
                        button(text("Light"))
                            .on_press(Message::AppearanceSelected(Appearance::Light)),
                        button(text("System"))
                            .on_press(Message::AppearanceSelected(Appearance::System)),
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
