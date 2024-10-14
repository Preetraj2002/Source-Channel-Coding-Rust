use std::char;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

// Reads a file and stores the frquency in a hashmap
// where key => character, value => frequency count
pub fn count_frequency(file_path: &str) -> Option<HashMap<char, u32>> {
    // initialize file reader and empty hashmap
    let mut file_char_freq: HashMap<char, u32> = HashMap::new();
    let f = File::open(file_path).ok()?;
    let reader = BufReader::new(f);

    // process lines
    for line in reader.lines() {
        let mut line_char_freq: HashMap<char, u32> = process_line(line.ok());
        merge_map(&mut file_char_freq, &line_char_freq);
    }

    Some(file_char_freq)
}

fn process_line(line: Option<String>) -> HashMap<char, u32> {
    let mut char_count = HashMap::new();
    if let Some(s) = line {
        s.chars().for_each(|c| {
            *char_count.entry(c).or_insert(0) += 1;
        });
    }
    char_count
}

// Merge 2 hashmaps by updating the common keys and
// add any keys that is introduced
fn merge_map(map1: &mut HashMap<char, u32>, map2: &HashMap<char, u32>) {
    map1.extend(map2.into_iter().map(|(k, v)| (k.clone(), v.clone())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line_with_characters() {
        let line = Some("hello world".to_string());
        let result = process_line(line);

        let mut expected = HashMap::new();
        expected.insert('h', 1);
        expected.insert('e', 1);
        expected.insert('l', 3);
        expected.insert('o', 2);
        expected.insert(' ', 1); // Space character count
        expected.insert('w', 1);
        expected.insert('r', 1);
        expected.insert('d', 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_process_line_empty() {
        let line = Some("".to_string());
        let result = process_line(line);

        let expected: HashMap<char, u32> = HashMap::new();
        assert_eq!(result, expected);
    }
}
