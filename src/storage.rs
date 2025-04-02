use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use serde_json::Value;

/// Storage struct to manage data persistence
#[derive(Clone)]
pub struct Storage {
    data: Arc<Mutex<Vec<(String, Value)>>>,
    storage_dir: PathBuf,
}

impl Storage {
    /// Creates a new Storage instance
    pub fn new() -> io::Result<Self> {
        // Create storage directory in user's home directory
        let mut storage_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        storage_dir.push(".dbug_desktop");

        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)?;
        }

        let data_file = storage_dir.join("data.json");
        let data = if data_file.exists() {
            // Load existing data
            let mut file = File::open(&data_file)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            serde_json::from_str::<Vec<(String, Value)>>(&contents).unwrap_or_else(|_| Vec::new())
        } else {
            Vec::new()
        };

        Ok(Self {
            data: Arc::new(Mutex::new(data)),
            storage_dir,
        })
    }

    /// Adds a JSON value to the storage
    pub fn add_json(&self, json: &Value) -> io::Result<()> {
        let id = Utc::now().timestamp_millis().to_string();

        // Add to memory
        {
            let mut data = self.data.lock().unwrap();
            data.push((id.clone(), json.clone()));
        }

        // Save to file
        self.save_to_file()
    }

    /// Retrieves all stored data
    pub fn get_all(&self) -> Vec<(String, Value)> {
        let data = self.data.lock().unwrap();
        data.clone()
    }

    /// Deletes an item by ID
    pub fn _delete(&self, id: &str) -> io::Result<bool> {
        let found;
        {
            let mut data = self.data.lock().unwrap();
            let len_before = data.len();
            data.retain(|(item_id, _)| {
                if item_id == id {
                    false // Remove this item
                } else {
                    true // Keep this item
                }
            });
            found = data.len() < len_before;
        }

        if found {
            self.save_to_file()?;
        }

        Ok(found)
    }

    /// Saves the current state to a file
    fn save_to_file(&self) -> io::Result<()> {
        let data_file = self.storage_dir.join("data.json");
        let data = self.data.lock().unwrap();

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(data_file)?;

        serde_json::to_writer_pretty(file, &*data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}