use serde_json::Value;
use std::collections::VecDeque;

pub struct Storage {
    data: VecDeque<Value>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: VecDeque::new(),
        }
    }

    pub fn add(&mut self, json: Value) {
        self.data.push_front(json);
    }

    pub fn get_all(&self) -> &VecDeque<Value> {
        &self.data
    }
}
