use crate::storage::Storage;
use iced::{Point, Size, Theme};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

impl Into<Point> for SerializablePoint {
    fn into(self) -> Point {
        Point::new(self.x, self.y)
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

impl Into<Size> for SerializableSize {
    fn into(self) -> Size {
        Size::new(self.width, self.height)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    theme_name: String,
    window_position: SerializablePoint,
    window_size: SerializableSize,
    // ... any other settings
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme_name: "Dark".to_string(),
            window_position: SerializablePoint { x: 200.0, y: 400.0 },
            window_size: SerializableSize {
                width: 400.0,
                height: 600.0,
            },
        }
    }
}

impl Settings {
    // Default settings with a built-in theme
    pub fn default() -> Self {
        Self {
            theme_name: "Dark".to_string(), // Default theme name
            window_position: Point::new(200.0, 400.0).into(),
            window_size: Size::new(400.0, 600.0).into(),
            // ... other default settings
        }
    }

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

    fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("dbug-desktop")
            .join("config.txt")
    }
}
