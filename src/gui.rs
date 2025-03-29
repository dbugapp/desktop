use iced::{Application, Command, Element, Column, Text};
use crate::storage::Storage;
use std::sync::{Arc, Mutex};

pub struct App {
    storage: Arc<Mutex<Storage>>,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            App {
                storage: Arc::new(Mutex::new(Storage::new())),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("JSON Viewer")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let content = self.storage.lock().unwrap().get_all().iter().fold(
            Column::new(),
            |column, json| {
                column.push(Text::new(format!("{}", json)))
            },
        );

        content.into()
    }
}
