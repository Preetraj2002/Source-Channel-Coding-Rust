use std::collections::HashMap;

pub fn create_lookup_table() -> (HashMap<char, usize>, HashMap<usize, char>) {
    let mut char_to_num = HashMap::new();
    let mut num_to_char = HashMap::new();

    // Add alphabets a-z
    for (i, c) in ('a'..='z').enumerate() {
        char_to_num.insert(c, i + 1); // 'a' -> 1, 'b' -> 2, etc.
        num_to_char.insert(i + 1, c); // 1 -> 'a', 2 -> 'b', etc.
    }

    // Add space character
    let space_index = 27; // Assign 27 to the space
    char_to_num.insert(' ', space_index);
    num_to_char.insert(space_index, ' ');

    // Add punctuation symbols
    let symbols = vec!['!', '?', ',', '.', ';', ':'];
    let start = 28; // Continue from where space ended

    for (i, s) in symbols.iter().enumerate() {
        char_to_num.insert(*s, start + i); // Assign numbers to symbols
        num_to_char.insert(start + i, *s); // Assign symbols to numbers
    }

    (char_to_num, num_to_char)
}

pub fn divide_into_chunks(text: &str, k: usize) -> Vec<String> {
    let mut chunks = Vec::new();

    // Split the text into chunks of size `k`
    for i in (0..text.len()).step_by(k) {
        let chunk = &text[i..std::cmp::min(i + k, text.len())]; // Ensure we don't go out of bounds
        chunks.push(chunk.to_string());
    }

    chunks
}
