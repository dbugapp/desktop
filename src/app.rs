use crate::server::ServerMessage;
use crate::settings::Settings;
use crate::storage::Storage;
use iced::event::Event;
use serde_json::Value;
use std::collections::HashSet;

// Add imports for global_hotkey
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};

// Add import for window::Id
use iced::window;

// Add Task import
use iced::Task;

/// Application state and logic
pub(crate) struct App {
    pub(crate) show_modal: bool,
    pub(crate) settings: Settings,
    pub(crate) storage: Storage,
    pub(crate) expanded_payload_id: Option<String>,
    pub(crate) collapsed_json_lines: HashSet<usize>,
    pub(crate) payload_list_cache: Vec<(String, Value)>,
    pub(crate) search_query: String,
    _hotkey_manager: GlobalHotKeyManager,
    pub(crate) main_window_id: Option<window::Id>,
}

impl App {
    pub(crate) fn default() -> (Self, Task<Message>) {
        let storage = Storage::new().expect("Failed to initialize storage");
        let payload_list_cache = storage.get_all();
        let newest_payload_id = payload_list_cache.first().map(|(id, _)| id.clone());

        // Initialize and register hotkey
        let manager = GlobalHotKeyManager::new().expect("Failed to create GlobalHotKeyManager");
        let hotkey = HotKey::new(Some(Modifiers::SHIFT | Modifiers::SUPER), Code::KeyL);
        // Just register, don't need to store ID for now
        if let Err(e) = manager.register(hotkey) {
            eprintln!("ERROR: Failed to register hotkey: {}", e);
        }

        /*
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            println!("{:?}", event);
        }
        */

        let app = Self {
            show_modal: false,
            settings: Settings::load(),
            storage,
            expanded_payload_id: newest_payload_id,
            collapsed_json_lines: HashSet::new(),
            payload_list_cache,
            search_query: String::new(),
            _hotkey_manager: manager,
            main_window_id: None,
        };

        // Return the App instance and an initial command (Task::none() here)
        (app, Task::none())
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
    HotkeyActivated(u32),
    CaptureWindowId(window::Id),
} 