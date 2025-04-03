use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// Enum representing the available themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Appearance {
    Dark,
    Light,
    System,
}

impl Default for Appearance {
    fn default() -> Self {
        Appearance::System
    }
}

/// Struct to manage application settings
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub appearance: Appearance,
}

impl Settings {
    /// Loads settings from the settings file
    pub fn load() -> Self {
        let path = Self::settings_path();

        if path.exists() {
            let mut file = match File::open(&path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Failed to open settings file: {}", err);
                    return Self::default();
                }
            };

            let mut contents = String::new();
            if let Err(err) = file.read_to_string(&mut contents) {
                eprintln!("Failed to read settings file: {}", err);
                return Self::default();
            }
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    /// Saves the current settings to a file
    pub fn save(&self) -> io::Result<()> {
        let path = Self::settings_path();

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    /// Returns the path to the settings file
    fn settings_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".dbug_desktop");
        path.push("settings.json");
        path
    }
}
