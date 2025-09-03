use chrono::Utc;
use serde_json::Value;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

// Define the final storage limit (2 MiB)
const MAX_STORAGE_BYTES: u64 = 2 * 1024 * 1024;

/// Helper function to estimate the size of a JSON value in bytes
/// Uses compact JSON representation length as an estimate.
fn estimate_payload_size(value: &Value) -> u64 {
    serde_json::to_string(value).unwrap_or_default().len() as u64
}

// Type alias for the data stored within the Mutex
type StorageState = (Vec<(String, Value, u64)>, u64);

/// Storage struct to manage data persistence
#[derive(Clone)]
pub struct Storage {
    // Use the type alias for clarity
    data: Arc<Mutex<StorageState>>,
    storage_dir: PathBuf,
}

impl Storage {
    /// Creates a new Storage instance
    pub fn new() -> io::Result<Self> {
        // Inlined logic from storage_dir_path()
        let storage_dir = dirs::home_dir()
            .map(|mut path| {
                path.push(".dbug_desktop");
                path
            })
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;

        // Inlined logic from ensure_storage_dir()
        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)?;
        }

        let data_file = storage_dir.join("data.json");
        let (initial_payloads, initial_total_bytes) = if data_file.exists() {
            let mut file = File::open(&data_file)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            match serde_json::from_str::<Vec<(String, Value)>>(&contents) {
                Ok(old_data) => {
                    let mut new_data = Vec::with_capacity(old_data.len());
                    let mut total_bytes: u64 = 0;
                    for (id, value) in old_data {
                        let size = estimate_payload_size(&value);
                        total_bytes += size;
                        new_data.push((id, value, size));
                    }
                    (new_data, total_bytes)
                }
                Err(_) => {
                    match serde_json::from_str::<Vec<(String, Value, u64)>>(&contents) {
                        Ok(parsed_data) => {
                            let total_bytes = parsed_data.iter().map(|(_, _, size)| size).sum();
                            eprintln!(
                                "INFO: Loaded {} existing payloads, total size: {} bytes",
                                parsed_data.len(),
                                total_bytes
                            );
                            (parsed_data, total_bytes)
                        }
                        Err(e) => {
                            eprintln!(
                                "WARN: Failed to parse data.json (both formats), starting fresh: {e}",
                            );
                            (Vec::new(), 0)
                        }
                    }
                }
            }
        } else {
            (Vec::new(), 0)
        };

        Ok(Self {
            data: Arc::new(Mutex::new((initial_payloads, initial_total_bytes))),
            storage_dir,
        })
    }

    pub fn config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".dbug_desktop")
            .join("config.json")
    }

    #[allow(clippy::map_unwrap_or)]
    pub fn save_config<T: serde::Serialize>(config: &T) -> io::Result<()> {
        let config_file = Self::config_path();
        let fallback = PathBuf::from(".");
        let dir = config_file.parent().unwrap_or(&fallback);

        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("ERROR: Failed to create config directory {dir:?}: {e}");
            return Err(e);
        }

        let file = match File::create(&config_file) {
            Ok(f) => {
                f
            }
            Err(e) => {
                eprintln!("ERROR: Failed to create/open config file {config_file:?}: {e}");
                return Err(e);
            }
        };

        let mut writer = io::BufWriter::new(file);

        match serde_json::to_writer_pretty(&mut writer, config) {
            Ok(()) => {
                match writer.flush() {
                    Ok(()) => {
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("ERROR: Failed to flush config file {config_file:?}: {e}");
                        Err(e)
                    }
                }
            }
            Err(e) => {
                eprintln!("ERROR: Failed to serialize/write config to file {config_file:?}: {e}");
                Err(io::Error::other(e))
            }
        }
    }

    #[allow(clippy::map_unwrap_or)] // Allow this pattern for clarity of fallback
    pub fn load_config<T: serde::de::DeserializeOwned + Default>() -> T {
        let config_file = dirs::home_dir()
            .map(|mut p| {
                p.push(".dbug_desktop");
                p.join("config.json")
            })
            .unwrap_or_else(|| PathBuf::from("./.dbug_desktop/config.json"));

        if let Ok(file) = File::open(&config_file) {
            serde_json::from_reader(file).unwrap_or_else(|e| {
                eprintln!(
                    "WARN: Failed to parse config file {config_file:?}, using defaults: {e}"
                );
                T::default()
            })
        } else {
            T::default()
        }
    }

    /// Adds a JSON value to the storage, enforcing size limit
    pub fn add_json(&self, json: &Value) -> io::Result<()> {
        let id = Utc::now().timestamp_millis().to_string();
        let new_payload_size = estimate_payload_size(json);

        match self.data.lock() {
            Ok(mut data_guard) => {
                let (payloads, current_total_bytes) = &mut *data_guard;

                // Enforce size limit - remove oldest entries first
                while !payloads.is_empty() &&
                      *current_total_bytes + new_payload_size > MAX_STORAGE_BYTES
                {
                    let removed_payload = payloads.remove(0);
                    let removed_size = removed_payload.2;
                    *current_total_bytes = current_total_bytes.saturating_sub(removed_size);
                }

                payloads.push((id.clone(), json.clone(), new_payload_size));
                *current_total_bytes += new_payload_size;
            }
            Err(poisoned) => {
                eprintln!("ERROR: Storage mutex poisoned in add_json: {poisoned}");
                return Err(io::Error::other("Mutex poisoned"));
            }
        }

        self.save_to_file()
    }

    /// Retrieves all stored data, mapping away the internal size
    pub fn get_all(&self) -> Vec<(String, Value)> {
        match self.data.lock() {
            Ok(data_guard) => {
                // Map to exclude the size before reversing and cloning
                data_guard.0.iter().map(|(id, value, _size)| (id.clone(), value.clone())).rev().collect()
            }
            Err(poisoned) => {
                eprintln!("ERROR: Storage mutex poisoned in get_all: {poisoned}");
                Vec::new()
            }
        }
    }

    /// Deletes an item by ID
    pub fn delete(&self, id: &str) -> io::Result<bool> {
        let mut requires_save = false;
        let mut deletion_happened = false;
        {
            let mut data_guard = match self.data.lock() {
                Ok(guard) => guard,
                Err(poisoned) => {
                    eprintln!("ERROR: Storage mutex poisoned in delete: {poisoned}");
                    return Err(io::Error::other("Mutex poisoned"));
                }
            };
            let (payloads, current_total_bytes) = &mut *data_guard;
            let len_before = payloads.len();
            let mut bytes_freed = 0;

            if let Some(payload_to_remove) = payloads.iter().find(|(item_id, _, _)| item_id == id) {
                 bytes_freed = payload_to_remove.2;
            }

            payloads.retain(|(item_id, _, _)| item_id != id);
            let deleted_now = payloads.len() < len_before;

            if deleted_now {
                *current_total_bytes = current_total_bytes.saturating_sub(bytes_freed);
                 eprintln!("INFO: Deleted payload ({}), freed {} bytes. New total: {}", id, bytes_freed, *current_total_bytes);
                 requires_save = true;
                 deletion_happened = true;
            }
        } // Mutex guard dropped here

        if requires_save {
            self.save_to_file()?;
        }

        Ok(deletion_happened)
    }

    /// Deletes all stored data
    pub fn delete_all(&self) -> io::Result<()> {
        match self.data.lock() {
            Ok(mut data_guard) => {
                let (payloads, current_total_bytes) = &mut *data_guard;
                if !payloads.is_empty() {
                     eprintln!("INFO: Clearing all {} payloads, freeing {} bytes.", payloads.len(), *current_total_bytes);
                     payloads.clear();
                     *current_total_bytes = 0;
                } else {
                    eprintln!("INFO: delete_all called but no payloads to clear.");
                }
            }
            Err(poisoned) => {
                eprintln!("ERROR: Storage mutex poisoned in delete_all: {poisoned}");
                return Err(io::Error::other("Mutex poisoned"));
            }
        }
        self.save_to_file()
    }

    /// Saves the current state (payloads only) to a file
    fn save_to_file(&self) -> io::Result<()> {
        let data_file = self.storage_dir.join("data.json");
        let data_to_save = match self.data.lock() {
            Ok(data_guard) => {
                // Clone the payload vec and map away the size before saving
                data_guard.0.iter()
                    .map(|(id, value, _size)| (id.clone(), value.clone()))
                    .collect::<Vec<_>>()
            }
            Err(poisoned) => {
                eprintln!("ERROR: Storage mutex poisoned during save_to_file: {poisoned}");
                return Err(io::Error::other("Mutex poisoned"));
            }
        };

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(data_file)?;

        let writer = io::BufWriter::new(file);

        // Serialize the mapped data (without sizes)
        serde_json::to_writer_pretty(writer, &data_to_save)
            .map_err(io::Error::other)
    }
}
