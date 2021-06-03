use std::fs;

use crate::error::Result;

pub fn create_dir(path: &String) -> Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn remove_file(filename: &String) -> Result<()> {
    fs::remove_file(filename)?;
    Ok(())
}

pub fn rename_file(src: &String, dest: &String) -> Result<()> {
    fs::rename(src, dest)?;
    Ok(())
}

pub fn lock_file(filename: &String) -> Result<()> {
    println!("lock file {}", filename);
    Ok(())
}

pub fn unlock_file(filename: &String) -> Result<()> {
    println!("unlock file {}", filename);
    Ok(())
}

pub fn file_exists(filename: &String) -> bool {
    fs::metadata(filename).is_ok()
}