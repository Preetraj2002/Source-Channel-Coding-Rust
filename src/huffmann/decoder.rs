use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Read, Write};

pub fn decompress(input_file: &str, output_file: &str) -> io::Result<()> {
    let mut file = File::open(input_file)?;
    let mut compressed_data = Vec::new();
    file.read_to_end(&mut compressed_data)?;

    // Find the start of the Huffman code table (header)
    let code_table_start = compressed_data
        .iter()
        .rposition(|&x| x == b'\n')
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No header found"))?;

    let code_table_json = &compressed_data[code_table_start + 1..];
    let huffman_codes: HashMap<String, char> = match serde_json::from_slice(code_table_json) {
        Ok(table) => table,
        Err(e) => {
            eprintln!("Failed to parse Huffman code table: {:?}", e);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Huffman code table",
            ));
        }
    };

    // Extract and decode the bit data
    let compressed_data = &compressed_data[..code_table_start];
    let mut bit_string = String::new();
    for &byte in compressed_data {
        bit_string.push_str(&format!("{:08b}", byte));
    }

    // Decode bit string using the Huffman codes
    let mut result = String::new();
    let mut temp = String::new();

    for bit in bit_string.chars() {
        temp.push(bit);
        if let Some(&ch) = huffman_codes.get(&temp) {
            result.push(ch);
            temp.clear();
        }
    }

    // Write the decompressed content to the output file
    let mut writer = BufWriter::new(File::create(output_file)?);
    writer.write_all(result.as_bytes())?;
    writer.flush()?;

    println!("File decompressed successfully.");
    Ok(())
}
