use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};

pub fn encoded_input(input: &str, huffmann_codes: &HashMap<char, String>) -> String {
    input
        .chars()
        .filter_map(|c| huffmann_codes.get(&c).cloned())
        .collect()
}

pub fn compress(
    encoded_data: String,
    huffman_codes: &HashMap<char, String>,
    output_file: &str,
) -> io::Result<()> {
    let mut buffer = Vec::new();
    let mut byte_accumulator = 0u8;
    let mut bit_count = 0;

    for bit in encoded_data.chars() {
        byte_accumulator <<= 1;
        if bit == '1' {
            byte_accumulator |= 1;
        }
        bit_count += 1;

        if bit_count == 8 {
            buffer.push(byte_accumulator);
            byte_accumulator = 0;
            bit_count = 0;
        }
    }

    if bit_count > 0 {
        byte_accumulator <<= 8 - bit_count;
        buffer.push(byte_accumulator);
    }

    // Convert Huffman codes to HashMap<String, char> for JSON serialization
    let huffman_codes_as_str: HashMap<String, char> = huffman_codes
        .iter()
        .map(|(&ch, code)| (code.clone(), ch))
        .collect();

    let mut writer = BufWriter::new(File::create(output_file)?);
    writer.write_all(&buffer)?;
    writer.write_all(b"\n")?;

    // Serialize and write the Huffman code table
    let header = serde_json::to_string(&huffman_codes_as_str)?;
    writer.write_all(header.as_bytes())?;
    writer.flush()?;

    // println!("File compressed successfully.");
    Ok(())
}

