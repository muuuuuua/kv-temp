use std::collections::HashMap;

pub struct MemTable {
    table: HashMap<String, String>,
}

impl MemTable {
    pub fn new() -> Self {
        MemTable { table: Default::default() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.table.insert(key, value);
    }

    pub fn get(&mut self, key: &String) -> Option<&String> {
        self.table.get(key)
    }
}
