# zigzag-rs

A dependency-free (including no std) ZigZag encoding/decoding Rust library. ZigZag encoding is a method for mapping signed integers to unsigned integers, commonly used in variable-length encoding and data compression.

## Features

- Completely dependency-free, usable in `#![no_std]` environments
- Supports all Rust native signed integer types (i8, i16, i32, i64, i128)
- Simple and easy-to-use API
- Efficient implementation

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
zigzag-rs = "0.1.0"
```

Example code:

```rust
use zigzag_rs::ZigZag;

fn main() {
    // Encoding
    let encoded = i32::zigzag_encode(-1);
    assert_eq!(encoded, 1u32);
    
    // Decoding
    let decoded = i32::zigzag_decode(1u32);
    assert_eq!(decoded, -1i32);
}
```

## ZigZag Encoding Principle

ZigZag encoding maps signed integers to unsigned integers as follows:
- 0 -> 0
- -1 -> 1
- 1 -> 2
- -2 -> 3
- 2 -> 4
...

This encoding method ensures that small absolute values (whether positive or negative) are mapped to small unsigned integers, which is ideal for subsequent variable-length encoding.

## License

MIT or Apache-2.0 (dual licensed) 