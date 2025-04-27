use crate::server::ServerMessage;
use crate::settings::Settings;
use crate::storage::Storage;
use iced::event::Event;
use serde_json::Value;
use std::collections::HashSet;

/// Application state and logic
pub(crate) struct App {
    pub(crate) show_modal: bool,
    pub(crate) settings: Settings,
    pub(crate) storage: Storage,
    pub(crate) expanded_payload_id: Option<String>,
    pub(crate) collapsed_json_lines: HashSet<usize>,
    pub(crate) payload_list_cache: Vec<(String, Value)>,
    pub(crate) search_query: String,
}

impl Default for App {
    fn default() -> Self {
        let storage = Storage::new().expect("Failed to initialize storage");
        let payload_list_cache = storage.get_all();
        let newest_payload_id = payload_list_cache.first().map(|(id, _)| id.clone());

        Self {
            show_modal: false,
            settings: Settings::load(),
            storage,
            expanded_payload_id: newest_payload_id,
            collapsed_json_lines: HashSet::new(),
            payload_list_cache,
            search_query: String::new(),
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
    TogglePayload(String),
    ToggleJsonSection(usize),
    ClearPayloads,
    DeletePayload(String),
    WindowMoved(iced::Point),
    WindowResized(iced::Size),
    WindowClosed,
    SearchQueryChanged(String),
} 