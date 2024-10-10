# Rust Source and Channel Coding Library

## Overview

This Rust library implements two essential coding techniques for data transmission:

1. **Source Coding** (Huffman Coding): A method of data compression that reduces redundancy in the source message, using variable-length codes to represent frequent symbols with shorter codes.
  
2. **Channel Coding** (Binary BCH Coding): A method of error correction used in communication systems. Binary BCH (Bose–Chaudhuri–Hocquenghem) codes detect and correct errors in transmitted data to ensure reliability over noisy channels.

## Features

- **Huffman Encoding and Decoding** for efficient data compression.
- **Binary BCH Encoding and Decoding** for error correction in communication channels.
- Written in Rust, ensuring performance and memory safety.
- Fully documented and tested with examples.

## Installation

To use this library in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
coding_lib = "0.1.0"
```

_Note: Replace `coding_lib` with the actual name once you publish it._

## Usage

### Huffman Coding (Source Coding)

```rust
use coding_lib::huffman;

fn main() {
    let data = "hello, world!";
    let (encoded, tree) = huffman::encode(data);
    let decoded = huffman::decode(&encoded, &tree);
    
    println!("Original: {}", data);
    println!("Encoded: {:?}", encoded);
    println!("Decoded: {}", decoded);
}
```

### Binary BCH Coding (Channel Coding)

```rust
use coding_lib::bch;

fn main() {
    let message = vec![1, 0, 1, 1]; // Binary message
    let encoded = bch::encode(&message);
    let decoded = bch::decode(&encoded).expect("Failed to decode");

    println!("Original: {:?}", message);
    println!("Encoded: {:?}", encoded);
    println!("Decoded: {:?}", decoded);
}
```

## Examples

Check the `examples` directory for more use cases and detailed examples.

## Testing

To run the tests, use:

```bash
cargo test
```

This will run the unit tests for both Huffman and BCH implementations.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request with improvements or bug fixes.

## License

This project is licensed under the MIT License.

