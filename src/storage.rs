use chrono::Utc;
use serde_json::Value;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};

/// Storage struct to manage data persistence
#[derive(Clone)]
pub struct Storage {
    data: Arc<Mutex<Vec<(String, Value)>>>,
    storage_dir: PathBuf,
}

impl Storage {
    /// Creates a new Storage instance
    pub fn new() -> io::Result<Self> {
        let mut storage_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
        storage_dir.push(".dbug_desktop");

        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)?;
        }

        let data_file = storage_dir.join("data.json");
        let data = if data_file.exists() {
            let mut file = File::open(&data_file)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            serde_json::from_str::<Vec<(String, Value)>>(&contents).unwrap_or_else(|e| {
                eprintln!(
                    "WARN: Failed to parse data file {:?}, starting fresh: {}",
                    data_file,
                    e
                );
                Vec::new()
            })
        } else {
            Vec::new()
        };

        Ok(Self {
            data: Arc::new(Mutex::new(data)),
            storage_dir,
        })
    }

    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".dbug_desktop")
            .join("config.json")
    }

    pub fn save_config<T: serde::Serialize>(config: &T) -> io::Result<()> {
        let config_file = Self::config_path();
        let fallback = PathBuf::from(".");
        let dir = config_file.parent().unwrap_or(&fallback);
        fs::create_dir_all(dir)?;

        let file = File::create(config_file)?;
        serde_json::to_writer_pretty(file, config)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    pub fn load_config<T: serde::de::DeserializeOwned + Default>() -> T {
        let config_file = Self::config_path();

        if let Ok(file) = File::open(config_file) {
            serde_json::from_reader(file).unwrap_or_default()
        } else {
            T::default()
        }
    }

    /// Adds a JSON value to the storage
    pub fn add_json(&self, json: &Value) -> io::Result<()> {
        let id = Utc::now().timestamp_millis().to_string();

        match self.data.lock() {
            Ok(mut data) => {
                data.push((id.clone(), json.clone()));
            }
            Err(poisoned) => {
                eprintln!("ERROR: Storage mutex poisoned in add_json: {}", poisoned);
                return Err(io::Error::new(io::ErrorKind::Other, "Mutex poisoned"));
            }
        }

        self.save_to_file()
    }

    /// Retrieves all stored data
    pub fn get_all(&self) -> Vec<(String, Value)> {
        match self.data.lock() {
            Ok(data) => data.iter().rev().cloned().collect(),
            Err(poisoned) => {
                eprintln!("ERROR: Storage mutex poisoned in get_all: {}", poisoned);
                Vec::new()
            }
        }
    }

    /// Deletes an item by ID
    pub fn delete(&self, id: &str) -> io::Result<bool> {
        let found;
        {
            let mut data = match self.data.lock() {
                Ok(guard) => guard,
                Err(poisoned) => {
                    eprintln!("ERROR: Storage mutex poisoned in delete: {}", poisoned);
                    return Err(io::Error::new(io::ErrorKind::Other, "Mutex poisoned"));
                }
            };
            let len_before = data.len();
            data.retain(|(item_id, _)| item_id != id);
            found = data.len() < len_before;
        }

        if found {
            self.save_to_file()?;
        }

        Ok(found)
    }

    /// Deletes all stored data
    pub fn delete_all(&self) -> io::Result<()> {
        match self.data.lock() {
            Ok(mut data) => {
                data.clear();
            }
            Err(poisoned) => {
                eprintln!("ERROR: Storage mutex poisoned in delete_all: {}", poisoned);
                return Err(io::Error::new(io::ErrorKind::Other, "Mutex poisoned"));
            }
        }
        self.save_to_file()
    }

    /// Saves the current state to a file
    fn save_to_file(&self) -> io::Result<()> {
        let data_file = self.storage_dir.join("data.json");
        let data_guard = match self.data.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("ERROR: Storage mutex poisoned in save_to_file: {}", poisoned);
                return Err(io::Error::new(io::ErrorKind::Other, "Mutex poisoned"));
            }
        };

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(data_file)?;

        let writer = io::BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &*data_guard)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}
