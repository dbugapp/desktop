use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use serde_json::Value;
use iced::futures::channel::mpsc;

use crate::storage_events::StorageCommand;

/// Storage struct to manage data persistence
#[derive(Clone)]
pub struct Storage {
    data: Arc<Mutex<Vec<(String, Value)>>>,
    storage_dir: PathBuf,
    event_sender: Arc<Mutex<Option<mpsc::Sender<StorageCommand>>>>,
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
            event_sender: Arc::new(Mutex::new(None)),
        })
    }

    pub fn set_event_sender(&mut self, sender: mpsc::Sender<StorageCommand>) {
        println!("Setting storage event sender");
        let mut event_sender = self.event_sender.lock().unwrap();
        *event_sender = Some(sender);
    }

    /// Adds a JSON value to the storage
    pub fn add_json(&self, json: Value) -> io::Result<()> {
        let id = Utc::now().timestamp_millis().to_string();
        
        // Add to memory
        {
            let mut data = self.data.lock().unwrap();
            data.push((id.clone(), json.clone()));
        }
        
        // Save to file
        self.save_to_file()?;

        // Notify about the change
        self.notify_updated();
        
        Ok(())
    }

    /// Retrieves all stored data
    pub fn get_all(&self) -> Vec<(String, Value)> {
        let data = self.data.lock().unwrap();
        data.clone()
    }

    /// Deletes an item by ID
    pub fn delete(&self, id: &str) -> io::Result<bool> {
        let found;
        {
            let mut data = self.data.lock().unwrap();
            let len_before = data.len();
            data.retain(|(item_id, _)| item_id != id);
            found = data.len() < len_before;
        }
        
        if found {
            self.save_to_file()?;
            
            // Notify about the change
            self.notify_updated();
        }
        
        Ok(found)
    }

    /// Helper method to send events to the event channel
    fn notify_updated(&self) {
        let event_sender = self.event_sender.lock().unwrap();
        if let Some(sender) = event_sender.as_ref() {
            let mut sender_clone = sender.clone();
            // Use tokio spawn to avoid blocking on send
            tokio::spawn(async move {
                if let Err(e) = sender_clone.try_send(StorageCommand::Updated) {
                    println!("Failed to send storage event: {}", e);
                } else {
                    println!("Successfully sent storage update event");
                }
            });
        } else {
            println!("Warning: No event sender available");
        }
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
