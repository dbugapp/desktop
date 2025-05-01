use crate::server::ServerMessage;
use crate::settings::Settings;
use crate::storage::Storage;
use iced::window;
use iced::Task;
use iced::event::Event;
use serde_json::Value;
use std::collections::HashSet;
use std::collections::HashMap;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum HotkeyAction {
    ShowWindow,
    ClearPayloads,
}

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
    pub(crate) hotkey_actions: HashMap<u32, HotkeyAction>,
}

impl App {
    pub(crate) fn default() -> (Self, Task<Message>) {
        let storage = Storage::new().expect("Failed to initialize storage");
        let payload_list_cache = storage.get_all();
        let newest_payload_id = payload_list_cache.first().map(|(id, _)| id.clone());

        let manager = GlobalHotKeyManager::new().expect("Failed to create GlobalHotKeyManager");
        let mut hotkey_actions = HashMap::new();

        let hotkey_visible = HotKey::new(Some(Modifiers::SHIFT | Modifiers::SUPER), Code::KeyL);
        let hotkey_clear_payloads = HotKey::new(Some(Modifiers::SHIFT | Modifiers::SUPER), Code::KeyK);

        hotkey_actions.insert(hotkey_visible.id, HotkeyAction::ShowWindow);
        hotkey_actions.insert(hotkey_clear_payloads.id, HotkeyAction::ClearPayloads);

        manager.register(hotkey_visible).expect("Failed to register ShowWindow hotkey");
        manager.register(hotkey_clear_payloads).expect("Failed to register ClearPayloads hotkey");

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
            hotkey_actions,
        };

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
    // Re-introduce HotkeyActivated
    HotkeyActivated(u32),
    CaptureWindowId(window::Id),
} 