use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
    System,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::System
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    pub theme: Theme,
}

impl Settings {
    pub fn load() -> Self {
        let path = Self::settings_path();
        
        if path.exists() {
            let mut file = File::open(&path).unwrap_or_else(|_| {
                panic!("Failed to open settings file")
            });
            
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap_or_else(|_| {
                panic!("Failed to read settings file")
            });
            
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }
    
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
    
    fn settings_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".dbug_desktop");
        path.push("settings.json");
        path
    }
}
