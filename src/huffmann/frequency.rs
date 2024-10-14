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
    #[test]
    fn test_merge_map_with_overlapping_keys() {
        let mut map1 = HashMap::new();
        map1.insert('a', 1);
        map1.insert('b', 2);

        let mut map2 = HashMap::new();
        map2.insert('b', 3);
        map2.insert('c', 4);

        merge_map(&mut map1, &map2);

        let mut expected = HashMap::new();
        expected.insert('a', 1);
        expected.insert('b', 3); // Value from map2 should overwrite the value from map1
        expected.insert('c', 4);

        assert_eq!(map1, expected);
    }

    #[test]
    fn test_merge_map_with_non_overlapping_keys() {
        let mut map1 = HashMap::new();
        map1.insert('x', 10);
        map1.insert('y', 20);

        let mut map2 = HashMap::new();
        map2.insert('z', 30);
        map2.insert('w', 40);

        merge_map(&mut map1, &map2);

        let mut expected = HashMap::new();
        expected.insert('x', 10);
        expected.insert('y', 20);
        expected.insert('z', 30);
        expected.insert('w', 40);

        assert_eq!(map1, expected);
    }

    #[test]
    fn test_merge_map_with_empty_map2() {
        let mut map1 = HashMap::new();
        map1.insert('a', 1);
        map1.insert('b', 2);

        let map2 = HashMap::new(); // Empty map

        merge_map(&mut map1, &map2);

        let mut expected = HashMap::new();
        expected.insert('a', 1);
        expected.insert('b', 2);

        assert_eq!(map1, expected); // map1 should remain unchanged
    }

    #[test]
    fn test_merge_map_with_empty_map1() {
        let mut map1 = HashMap::new(); // Empty map
        let mut map2 = HashMap::new();
        map2.insert('a', 1);
        map2.insert('b', 2);

        merge_map(&mut map1, &map2);

        let mut expected = HashMap::new();
        expected.insert('a', 1);
        expected.insert('b', 2);

        assert_eq!(map1, expected); // map1 should be updated with the contents of map2
    }

    #[test]
    fn test_merge_map_with_both_empty() {
        let mut map1 = HashMap::new(); // Empty map
        let map2 = HashMap::new(); // Empty map

        merge_map(&mut map1, &map2);

        let expected: HashMap<char, u32> = HashMap::new();
        assert_eq!(map1, expected); // Both maps are empty, map1 should remain empty
    }
}
