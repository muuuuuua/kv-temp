mod db;
mod error;
mod memtable;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut kv = db::KvStore::open("test".into()).unwrap();
        kv.put("a".into(), "b".into());
        kv.get("a".into());
        kv.delete("a".into());
        assert_eq!(2 + 2, 4);
    }
}
