use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{collections::HashMap, io};

pub fn count_frequency_from_file(file_path: &str) -> io::Result<HashMap<char, u32>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut freq_map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for c in line.chars() {
            *freq_map.entry(c).or_insert(0) += 1;
        }
        *freq_map.entry('\n').or_insert(0) += 1;
    }
    Ok(freq_map)
}
