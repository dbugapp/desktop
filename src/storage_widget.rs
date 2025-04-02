use crate::storage::{Storage, Message as StorageMessage};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Task};
use serde_json::Value;
use iced::widget::scrollable;


pub struct StorageWidget {
    storage: Storage,
    listener_id: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Delete(String),
    DataChanged,
}

impl StorageWidget {


    pub fn new(storage: Storage) -> Self {
        // Create a temporary widget first
        let mut widget = Self {
            storage: storage.clone(),
            listener_id: None,
        };

        // Register as a listener to storage changes
        let listener_id = storage.add_listener(|| StorageMessage::DataChanged);

        // Set the listener_id directly on the widget
        widget.listener_id = Some(listener_id);

        // Return the widget
        widget
    }



    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Delete(id) => {
                if let Err(e) = self.storage.delete(&id) {
                    eprintln!("Failed to delete item: {}", e);
                }
                Task::none()
            },
            Message::DataChanged => {
                Task::none()
            },
        }
    }
    
    pub fn view(&self) -> Element<Message> {
        let items = self.storage.get_all();
        let storage_rows = column(
            items.iter().map(|(id, value)| {
                let item_id = id.clone();
                container(
                    row![
                        text(format_json(value)).width(Length::Fill),
                        button(text("Delete")).on_press(Message::Delete(item_id))
                    ]
                        .spacing(10)
                        .width(Length::Fill)
                )
                    .padding(10)
                    .into()
            }).collect::<Vec<_>>()
        )
            .spacing(10)
            .padding(10);

        // Wrap the storage rows in a scrollable container
        scrollable(storage_rows)
            .width(Length::Fill)
            .height(Length::Fill).into()

    }
}

impl Drop for StorageWidget {
    fn drop(&mut self) {
        // Clean up by removing the listener when the widget is dropped
        if let Some(id) = &self.listener_id {
            self.storage.remove_listener(id);
        }
    }
}

// Helper function to format JSON nicely for display
fn format_json(value: &Value) -> String {
    match serde_json::to_string_pretty(value) {
        Ok(s) => s,
        Err(_) => format!("{:?}", value),
    }
}