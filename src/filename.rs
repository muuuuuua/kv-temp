pub fn lock_file_name(db_name: &String) -> String {
    format!("{}/LOCK", db_name)
}

pub fn current_file_name(db_name: &String) -> String {
    format!("{}/CURRENT", db_name)
}

pub fn descriptor_file_name(db_name: &String, number: u64) -> String {
    format!("{}/MANIFEST-{:06}", db_name, number)
}