use std::fs;

use crate::error::{KvError, Result};
use crate::filename::*;
use crate::filesystem::*;
use crate::memtable::{MemTable, ValueType};

pub struct KvStore {
    mem: MemTable,
}

impl KvStore {
    pub fn open(path: String) -> Result<Self> {
        if let Ok(kv) = KvStore::recover(&path) {
            Ok(kv)
        } else {
            KvStore::new(&path)
        }
    }

    pub fn put(&mut self, key: String, val: String) {
        self.mem.set(ValueType::TypeValue, key, val);
    }

    pub fn delete(&mut self, key: String) {
        self.mem.set(ValueType::TypeDeletion, key, "".into());
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        if let Some(x) = self.mem.get(key) {
            Some(x)
        } else {
            None
        }
    }

    fn recover(path: &String) -> Result<Self> {
        println!("try to recover from file {}", path);


        Err(KvError::InvalidArgument)
    }

    fn new(path: &String) -> Result<Self> {
        println!("begin to create new db {}", path);
        create_dir(path)?;
        // lock file
        if let Err(_) = lock_file(&lock_file_name(path)) {
            println!("lock file failed");
            return Err(KvError::IOError);
        }

        // new log file



        Ok(KvStore {
            mem: MemTable::new()
        })
    }
}

impl Drop for KvStore {
    fn drop(&mut self) {
        println!("drop kv store");
        let _ = unlock_file(&"".to_string());
    }
}