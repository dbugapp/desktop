use iced::event::{self, Event};
use iced::{keyboard, Length};
use iced::keyboard::key;
use iced::widget::{
    self, button, center, column, container, horizontal_space, mouse_area,
    opaque, row, stack, text, svg,
};
use iced::{Bottom, Color, Element, Fill, Subscription, Task};
use iced::futures::channel::mpsc;
use iced::widget::scrollable;

use crate::settings::{Settings, Theme};
use crate::storage::Storage;
use crate::storage_events::{StorageCommand, StorageEvent, storage_sipper};

/// Initializes and runs the GUI application
pub fn gui() -> iced::Result {
    iced::application("Modal - Iced", App::update, App::view)
        .subscription(App::subscription)
        .run()
}

/// Application state and logic
pub struct App {
    show_modal: bool,
    settings: Settings,
    storage: Storage,
    storage_sender: Option<mpsc::Sender<StorageCommand>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_modal: false,
            settings: Settings::load(),
            storage: Storage::new().expect("Failed to initialize storage"),
            storage_sender: None,
        }
    }
}

/// Messages used for application state updates
#[derive(Debug, Clone)]
enum Message {
    ShowModal,
    HideModal,
    ThemeSelected(Theme),
    Event(Event),
    StorageEvent(StorageEvent),
}

impl App {
    /// Subscribes to application events
    fn subscription(&self) -> Subscription<Message> {
        iced::Subscription::batch(vec![
            event::listen().map(Message::Event),
            iced::Subscription::run(|| storage_sipper()).map(Message::StorageEvent),
        ])
    }

    /// Updates application state based on messages
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
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
            Message::StorageEvent(event) => match event {
                StorageEvent::Connected(sender) => {
                    println!("Connected to storage event system");
                    self.storage_sender = Some(sender.clone());
                    // Connect the storage to the event system
                    self.storage.set_event_sender(sender);
                    Task::none()
                }
                StorageEvent::StorageUpdated => {
                    println!("Storage updated, refreshing UI");
                    Task::none()
                }
            }
        }
    }

    /// Renders the application view
    fn view(&self) -> Element<Message> {
        let handle = svg::Handle::from_path("src/assets/icons/mdi--mixer-settings.svg");

        // Create the storage rows column
        let storage_rows = column(
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
            .padding(10);

        // Wrap the storage rows in a scrollable container
        let scrollable_storage = scrollable(storage_rows)
            .width(Length::Fill)
            .height(Length::Fill);

        let content = container(
            column![
            row![
                horizontal_space(),
                button(svg(handle).width(20).height(20)).on_press(Message::ShowModal)
            ]
            .height(Length::Shrink),
            scrollable_storage,
            row![
                horizontal_space()
            ]
            .align_y(Bottom)
            .height(Length::Shrink),
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
