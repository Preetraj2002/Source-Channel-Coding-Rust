use std::{fs::metadata, io};

pub fn get_file_size(file_path: &str) -> io::Result<u64> {
    let meta = metadata(file_path)?;
    Ok(meta.len())
}