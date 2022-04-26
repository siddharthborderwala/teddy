use std::collections::HashMap;

use crate::query::QueryResult;
use crate::value::Value;

pub struct Teddy {
    pub map: HashMap<String, Value>,
}

impl Teddy {
    pub fn new() -> Self {
        Self {
            map: HashMap::<String, Value>::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> QueryResult {
        self.map
            .get(key.into())
            .cloned()
            .ok_or(format!("No value associated with key {}", key))
    }

    pub fn exists(&self, key: &str) -> bool {
        self.map.contains_key(key.into())
    }

    pub fn delete(&mut self, key: &str) -> QueryResult {
        self.map.remove(key.into()).ok_or(format!(
            "Cannot delete value for key {} as it does not exist",
            key
        ))
    }
}

// perform some validation in here and could return Result enum instead
// maybe add serialization to JSON using serde
