use crate::error::{Result, KvError};
use crate::memtable::MemTable;

pub struct KvStore {
    mem: MemTable
}

impl KvStore {
     pub fn open(path: String) -> Result<Self> {
        if let Ok(kv) = KvStore::recover(&path) {
            Ok(kv)
        } else {
            KvStore::new(&path)
        }
    }

    pub fn put(&mut self, key: String, val: String) {}

    pub fn delete(&mut self, key: String) {}

    pub fn get(&mut self, key: String) {}

    fn recover(path: &String) -> Result<Self> {
        println!("try to recover from file {}", path);
        Err(KvError::InvalidArgument)
    }

    fn new(path: &String) -> Result<Self> {
        println!("try to create new db {}", path);
        Ok(KvStore { mem: MemTable::new() })
    }
}