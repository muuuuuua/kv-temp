mod db;
mod error;
mod memtable;
mod filename;
mod filesystem;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut kv = db::KvStore::open("hehehou".into()).unwrap();
        assert_eq!(kv.get("a".into()), None);
        kv.put("a".into(), "b".into());
        assert_eq!(kv.get("a".into()), Some("b".to_string()));
        kv.delete("a".into());
        assert_eq!(kv.get("a".into()), None);
    }
}
