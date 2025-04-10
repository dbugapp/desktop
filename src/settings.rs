use iced::Theme;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    // Store the theme name as a string
    theme_name: String,
    // ... any other settings
}

impl Settings {
    // Default settings with a built-in theme
    pub fn default() -> Self {
        Self {
            theme_name: "Dark".to_string(), // Default theme name
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

    // Loads settings from storage
    pub fn load() -> Self {
        let path = Self::path();

        match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|err| {
                eprintln!("Error parsing settings: {}", err);
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    // Saves settings to storage
    pub fn save(&self) -> Result<(), String> {
        let path = Self::path();

        // Create directory if it doesn't exist
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }

        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())?;

        Ok(())
    }

    // Path to settings file
    fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("dbug-desktop")
            .join("settings.json")
    }
}
