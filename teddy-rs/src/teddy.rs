///! This file holds everything pertaining to the main Teddy struct
///! and includes the implementation of all different operations
///! supported by Teddy
use std::collections::HashMap;

use crate::query::QueryResult;
use crate::value::Value;

/// The Teddy struct
pub struct Teddy {
    pub map: HashMap<String, Value>,
}

impl Teddy {
    /// Create a new instance of Teddy
    pub fn new() -> Self {
        Self {
            map: HashMap::<String, Value>::new(),
        }
    }

    /// Set a new key value pair in the store
    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.into(), value.into());
    }

    /// Gets the value for a key from the store if it exists
    pub fn get(&self, key: &str) -> QueryResult {
        self.map
            .get(key.into())
            .cloned()
            .ok_or(format!("No value associated with key {}", key))
    }

    /// Returns true if the store has the specified key
    pub fn exists(&self, key: &str) -> bool {
        self.map.contains_key(key.into())
    }

    /// Deletes the key value pair from the store if exists
    pub fn delete(&mut self, key: &str) -> QueryResult {
        self.map.remove(key.into()).ok_or(format!(
            "Cannot delete value for key {} as it does not exist",
            key
        ))
    }
}
