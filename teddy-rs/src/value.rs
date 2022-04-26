use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    String(String),
    List(VecDeque<String>),
    Hash(HashMap<String, String>),
    Set(HashSet<String>),
    OrderedSet(BTreeSet<String>),
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl Value {
    pub fn create_list(values: Vec<&str>) -> Self {
        Self::List(
            values
                .into_iter()
                .map(|v| v.to_string())
                .collect::<VecDeque<String>>(),
        )
    }

    pub fn create_set(values: Vec<&str>) -> Self {
        Self::Set(
            values
                .into_iter()
                .map(|v| v.to_string())
                .collect::<HashSet<String>>(),
        )
    }

    pub fn create_ordered_set(values: Vec<&str>) -> Self {
        Self::OrderedSet(
            values
                .into_iter()
                .map(|v| v.to_string())
                .collect::<BTreeSet<String>>(),
        )
    }

    pub fn create_hash(values: Vec<&str>) -> Self {
        let mut map = HashMap::with_capacity(values.len() / 2);
        let mut cur_key = String::new();
        for (i, v) in values.into_iter().enumerate() {
            if i % 2 == 0 {
                cur_key = v.into();
            } else {
                map.insert(v.into(), cur_key.clone());
            }
        }
        Self::Hash(map)
    }
}
