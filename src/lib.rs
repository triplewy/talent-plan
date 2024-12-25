#![deny(missing_docs)]
//! This is documentation for the crate

use std::collections::HashMap;

/// Doc comment
pub struct KvStore {
    map: HashMap<String, String>,
}

impl Default for KvStore {
    /// Doc comment
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// Doc comment
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Doc comment
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Doc comment
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).map(|value| value.to_owned())
    }

    /// Doc comment
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
