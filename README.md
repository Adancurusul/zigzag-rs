# zigzag-rs
[![crates.io](https://img.shields.io/crates/v/zigzag-rs.svg)](https://crates.io/crates/zigzag-rs)

A dependency-free (including no std) ZigZag encoding/decoding Rust library. ZigZag encoding is a method for mapping signed integers to unsigned integers, commonly used in variable-length encoding and data compression.

## Features

- Completely dependency-free, usable in `#![no_std]` environments
- Supports all Rust native signed integer types (i8, i16, i32, i64, i128)
- Simple and easy-to-use API with both single value and batch processing
- Iterator-based API for memory-constrained environments
- Efficient implementation optimized for embedded systems
- Error handling with Result types for robust application development

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
zigzag-rs = "0.2.1"
```

### Single value encoding/decoding

```rust
use zigzag_rs::ZigZag;

// Encoding
let encoded = i32::zigzag_encode(-1);
assert_eq!(encoded, 1u32);

// Decoding
let decoded = i32::zigzag_decode(1u32);
assert_eq!(decoded, -1i32);
```

### Batch processing

```rust
use zigzag_rs::ZigZag;

// Prepare data
let values = [-10, -1, 0, 1, 10];
let mut encoded = [0u32; 5];
let mut decoded = [0i32; 5];

// Encode a slice of values
i32::zigzag_encode_slice(&values, &mut encoded);

// Decode a slice of values
i32::zigzag_decode_slice(&encoded, &mut decoded);

// Verify round-trip conversion
assert_eq!(values, decoded);
```

### Iterator-based API

The library provides an iterator-based API that encodes or decodes values on-the-fly as the iterator is consumed, without requiring an intermediate buffer:

```rust
use zigzag_rs::{ZigZag, zigzag_encode_iter, zigzag_decode_iter};

// Source data
let values = [-10, -1, 0, 1, 10];

// Create an iterator that encodes values on-demand
let encoded_iter = zigzag_encode_iter::<i32, _>(values.iter());

// Process encoded values without allocating a buffer
for (original, encoded) in values.iter().zip(encoded_iter) {
    println!("{} encodes to {}", original, encoded);
}

// For decoding
let encoded_values = [1u32, 0, 2, 3, 20];
let decoded_iter = zigzag_decode_iter::<i32, _>(encoded_values.iter());

// Process decoded values one at a time
for decoded in decoded_iter {
    // Use the decoded value...
}
```

This approach is particularly useful in memory-constrained environments like embedded systems.

### Error handling

The library provides error handling variants of the batch processing functions:

```rust
use zigzag_rs::ZigZag;

let values = [-10, -1, 0, 1, 10];
let mut encoded = [0u32; 5];

// Try to encode, but return a Result instead of panicking if the buffer is too small
let result = i32::try_zigzag_encode_slice(&values, &mut encoded);
if let Err(err) = result {
    println!("Buffer too small: needed {} but had {}", err.needed(), err.actual());
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

## Performance

The implementation is optimized for both single value processing and batch operations, making it suitable for resource-constrained environments like embedded systems.

## License

MIT or Apache-2.0 (dual licensed) 