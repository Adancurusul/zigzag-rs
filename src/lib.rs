#![no_std]

//! # zigzag-rs
//!
//! A dependency-free (including no std) ZigZag encoding/decoding Rust library.
//! ZigZag encoding is a method for mapping signed integers to unsigned integers,
//! commonly used in variable-length encoding and data compression.
//!
//! ## Features
//!
//! - Completely dependency-free, usable in `#![no_std]` environments
//! - Supports all Rust native signed integer types (i8, i16, i32, i64, i128)
//! - Simple and easy-to-use API
//! - Efficient implementation
//!
//! ## Usage
//!
//! Add the dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! zigzag-rs = "0.1.0"
//! ```
//!
//! Example code:
//!
//! ```rust
//! use zigzag_rs::ZigZag;
//!
//! fn main() {
//!     // Encoding
//!     let encoded = i32::zigzag_encode(-1);
//!     assert_eq!(encoded, 1u32);
//!     
//!     // Decoding
//!     let decoded = i32::zigzag_decode(1u32);
//!     assert_eq!(decoded, -1i32);
//! }
//! ```
//!
//! ## ZigZag Encoding Principle
//!
//! ZigZag encoding maps signed integers to unsigned integers as follows:
//! - 0 -> 0
//! - -1 -> 1
//! - 1 -> 2
//! - -2 -> 3
//! - 2 -> 4
//! ...
//!
//! This encoding method ensures that small absolute values (whether positive or negative)
//! are mapped to small unsigned integers, which is ideal for subsequent variable-length encoding.
/// Trait for ZigZag encoding, used to convert signed integers to unsigned integers
pub trait ZigZag {
    /// The corresponding unsigned type
    type UInt;
    
    /// Encode a signed integer to an unsigned integer
    fn zigzag_encode(value: Self) -> Self::UInt;
    
    /// Decode an unsigned integer back to a signed integer
    fn zigzag_decode(value: Self::UInt) -> Self;
}

macro_rules! impl_zigzag {
    ($signed:ty, $unsigned:ty, $bits:expr) => {
        impl ZigZag for $signed {
            type UInt = $unsigned;
            
            #[inline]
            fn zigzag_encode(value: Self) -> Self::UInt {
                // Left shift by one bit, then XOR with arithmetic right shift result
                ((value << 1) ^ (value >> ($bits - 1))) as $unsigned
            }
            
            #[inline]
            fn zigzag_decode(value: Self::UInt) -> Self {
                // Right shift by one bit
                let shr1 = (value >> 1) as $signed;
                // Lowest bit is 1 for negative, 0 for positive
                let neg_mask = -((value & 1) as $signed);
                // XOR operation to restore the original value
                shr1 ^ neg_mask
            }
        }
    };
}

// Implement ZigZag trait for various integer types
impl_zigzag!(i8, u8, 8);
impl_zigzag!(i16, u16, 16);
impl_zigzag!(i32, u32, 32);
impl_zigzag!(i64, u64, 64);
impl_zigzag!(i128, u128, 128);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encode_decode_i32() {
        // Test specific values
        assert_eq!(i32::zigzag_encode(0), 0u32);
        assert_eq!(i32::zigzag_encode(-1), 1u32);
        assert_eq!(i32::zigzag_encode(1), 2u32);
        assert_eq!(i32::zigzag_encode(-2), 3u32);
        
        // Test boundary values
        assert_eq!(i32::zigzag_encode(i32::MAX), 4294967294u32);
        assert_eq!(i32::zigzag_encode(i32::MIN), 4294967295u32);
        
        // Test round-trip conversion
        for i in [-100, -10, -1, 0, 1, 10, 100].iter() {
            let encoded = i32::zigzag_encode(*i);
            let decoded = i32::zigzag_decode(encoded);
            assert_eq!(*i, decoded);
        }
    }
    
    #[test]
    fn test_encode_decode_i8() {
        // Test round-trip conversion for i8 type
        for i in i8::MIN..=i8::MAX {
            let encoded = i8::zigzag_encode(i);
            let decoded = i8::zigzag_decode(encoded);
            assert_eq!(i, decoded);
        }
    }
    
    #[test]
    fn test_encode_decode_i16() {
        // Test some i16 values
        for i in [-1000, -100, -1, 0, 1, 100, 1000].iter() {
            let encoded = i16::zigzag_encode(*i);
            let decoded = i16::zigzag_decode(encoded);
            assert_eq!(*i, decoded);
        }
    }
}
