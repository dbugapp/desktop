use crate::storage::Storage;
use iced::{Point, Size, Theme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializablePoint {
    pub x: f32,
    pub y: f32,
}

impl From<Point> for SerializablePoint {
    fn from(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<SerializablePoint> for Point {
    fn from(val: SerializablePoint) -> Self {
        Point::new(val.x, val.y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSize {
    pub width: f32,
    pub height: f32,
}

impl From<Size> for SerializableSize {
    fn from(size: Size) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
    }
}

impl From<SerializableSize> for Size {
    fn from(val: SerializableSize) -> Self {
        Size::new(val.width, val.height)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    theme_name: String,
    window_position: SerializablePoint,
    window_size: SerializableSize,
    server_host: String,
    server_port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme_name: "Catppuccin Mocha".to_string(),
            window_position: SerializablePoint { x: 200.0, y: 400.0 },
            window_size: SerializableSize {
                width: 1280.0,
                height: 800.0,
            },
            server_host: "127.0.0.1".to_string(),
            server_port: 53821,
        }
    }
}

impl Settings {
    // Property to get the actual Theme
    pub fn theme(&self) -> Theme {
        // Find the theme by name in Theme::ALL, fallback to Dark
        Theme::ALL
            .iter()
            .find(|t| t.to_string() == self.theme_name)
            .cloned()
            .unwrap_or_else(|| {
                eprintln!("Theme '{}' not found, using Dark", self.theme_name);
                Theme::Dark
            })
    }

    // Method to set the theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme_name = theme.to_string();
    }

    pub fn load() -> Self {
        Storage::load_config()
    }

    pub fn save(&self) -> Result<(), String> {
        Storage::save_config(self).map_err(|e| e.to_string())
    }

    pub fn set_window_position(&mut self, position: Point) {
        self.window_position = position.into();
    }

    pub fn set_window_size(&mut self, size: Size) {
        self.window_size = size.into();
    }

    pub fn get_window_position(&self) -> Point {
        self.window_position.clone().into()
    }

    pub fn get_window_size(&self) -> Size {
        self.window_size.clone().into()
    }

    pub fn get_server_host(&self) -> &str {
        &self.server_host
    }

    pub fn get_server_port(&self) -> u16 {
        self.server_port
    }

    pub fn set_server_host(&mut self, host: String) {
        self.server_host = host;
    }

    pub fn set_server_port(&mut self, port: u16) {
        self.server_port = port;
    }
}
