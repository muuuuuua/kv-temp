use std::collections::HashMap;

pub enum ValueType {
    TypeDeletion = 0,
    TypeValue = 1
}

pub struct MemTable {
    table: HashMap<String, (ValueType, String)>,
}

impl MemTable {
    pub fn new() -> Self {
        MemTable { table: Default::default() }
    }

    pub fn set(&mut self, value_type: ValueType, key: String, value: String) {
        self.table.insert(key, (value_type, value));
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        match self.table.get(&key) {
            Some((ValueType::TypeDeletion, _)) => None,
            Some((_, value)) => Some(value.clone()),
            None => None,
        }
    }
}
