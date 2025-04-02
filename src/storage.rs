use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use serde_json::Value;

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

// Type for callback functions that notify listeners
type ListenerCallback = Box<dyn Fn() -> Message + Send + Sync>;

// Type for the actual message that will be sent
#[derive(Debug, Clone)]
pub enum Message {
    DataChanged,
    // You can add more specific message types as needed
}

/// Storage struct to manage data persistence
#[derive(Clone)]
pub struct Storage {
    data: Arc<Mutex<Vec<(String, Value)>>>,
    storage_dir: PathBuf,
    listeners: Arc<Mutex<HashMap<String, ListenerCallback>>>,
    next_listener_id: Arc<AtomicUsize>,
}

// In storage.rs
impl Message {
    pub fn to_widget_message(&self) -> crate::storage_widget::Message {
        match self {
            Message::DataChanged => crate::storage_widget::Message::DataChanged,
        }
    }
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
            listeners: Arc::new(Mutex::new(HashMap::new())),
            next_listener_id: Arc::new(AtomicUsize::new(0)),
        })
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
        self.notify_listeners();
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
            self.notify_listeners();
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

    pub fn add_listener<F, M>(&self, callback: F) -> String
    where
        F: Fn() -> M + Send + Sync + 'static,
        M: 'static,
    {
        let id = self.next_listener_id.fetch_add(1, Ordering::SeqCst).to_string();

        let mut listeners = self.listeners.lock().unwrap();
        listeners.insert(id.clone(), Box::new(move || Message::DataChanged));

        id
    }

    /// Removes a listener
    pub fn remove_listener(&self, id: &str) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.remove(id);
    }

    /// Notifies all listeners about data changes
    fn notify_listeners(&self) {
        let listeners = self.listeners.lock().unwrap();
        for callback in listeners.values() {
            callback();
        }
    }

}
