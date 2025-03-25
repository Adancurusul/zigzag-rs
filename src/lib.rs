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
//! - Simple and easy-to-use API with both single value and batch processing
//! - Zero-copy iterator API for memory-constrained environments
//! - Efficient implementation optimized for embedded systems
//! - Error handling with Result types for robust application development
//!
//! ## Usage
//!
//! Add the dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! zigzag-rs = "0.2.0"
//! ```
//!
//! Example code:
//!
//! ```rust
//! use zigzag_rs::ZigZag;
//!
//! fn main() {
//!     // Single value encoding/decoding
//!     let encoded = i32::zigzag_encode(-1);
//!     assert_eq!(encoded, 1u32);
//!     
//!     let decoded = i32::zigzag_decode(1u32);
//!     assert_eq!(decoded, -1i32);
//!     
//!     // Batch processing
//!     let values = [-10, -1, 0, 1, 10];
//!     let mut encoded = [0u32; 5];
//!     i32::zigzag_encode_slice(&values, &mut encoded);
//!     
//!     let mut decoded = [0i32; 5];
//!     i32::zigzag_decode_slice(&encoded, &mut decoded);
//!     
//!     assert_eq!(values, decoded);
//!     
//!     // Using Result-based error handling
//!     let values = [-10, -1, 0, 1, 10];
//!     let mut encoded = [0u32; 5];
//!     let result = i32::try_zigzag_encode_slice(&values, &mut encoded);
//!     assert!(result.is_ok());
//!     
//!     // Using zero-copy iterator API
//!     let values = [-10, -1, 0, 1, 10];
//!     // Encode each value on the fly without allocating a buffer
//!     let encoded_iter = zigzag_rs::zigzag_encode_iter::<i32, _>(values.iter());
//!     
//!     // The values are encoded only when the iterator is consumed
//!     for (original, encoded) in values.iter().zip(encoded_iter) {
//!         assert_eq!(encoded, i32::zigzag_encode(*original));
//!     }
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

/// Error type for ZigZag operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZigZagError {
    /// Output buffer is too small to hold all converted values
    BufferTooSmall {
        /// Number of elements needed
        needed: usize,
        /// Actual buffer size
        actual: usize,
    },
}

/// Trait for ZigZag encoding, used to convert signed integers to unsigned integers
pub trait ZigZag {
    /// The corresponding unsigned type
    type UInt;
    
    /// Encode a signed integer to an unsigned integer
    fn zigzag_encode(value: Self) -> Self::UInt;
    
    /// Decode an unsigned integer back to a signed integer
    fn zigzag_decode(value: Self::UInt) -> Self;
    
    /// Encode a slice of signed integers to unsigned integers
    /// 
    /// # Arguments
    /// * `values` - Slice of signed integers to encode
    /// * `out` - Output slice to store encoded unsigned integers
    /// 
    /// # Panics
    /// Panics if `out` is smaller than `values` 
    fn zigzag_encode_slice(values: &[Self], out: &mut [Self::UInt]) 
    where 
        Self: Sized + Copy
    {
        assert!(out.len() >= values.len(), "Output slice must be at least as large as input slice");
        for (i, &value) in values.iter().enumerate() {
            out[i] = Self::zigzag_encode(value);
        }
    }
    
    /// Decode a slice of unsigned integers back to signed integers
    /// 
    /// # Arguments
    /// * `values` - Slice of unsigned integers to decode
    /// * `out` - Output slice to store decoded signed integers
    /// 
    /// # Panics
    /// Panics if `out` is smaller than `values`
    fn zigzag_decode_slice(values: &[Self::UInt], out: &mut [Self]) 
    where 
        Self: Sized + Copy,
        Self::UInt: Copy
    {
        assert!(out.len() >= values.len(), "Output slice must be at least as large as input slice");
        for (i, &value) in values.iter().enumerate() {
            out[i] = Self::zigzag_decode(value);
        }
    }
    
    /// Try to encode a slice of signed integers to unsigned integers, returning
    /// a Result instead of panicking if the output buffer is too small
    /// 
    /// # Arguments
    /// * `values` - Slice of signed integers to encode
    /// * `out` - Output slice to store encoded unsigned integers
    /// 
    /// # Returns
    /// * `Ok(())` if all values were encoded successfully
    /// * `Err(ZigZagError::BufferTooSmall)` if output buffer is too small
    fn try_zigzag_encode_slice(values: &[Self], out: &mut [Self::UInt]) -> Result<(), ZigZagError> 
    where 
        Self: Sized + Copy
    {
        if out.len() < values.len() {
            return Err(ZigZagError::BufferTooSmall { 
                needed: values.len(), 
                actual: out.len(),
            });
        }
        
        for (i, &value) in values.iter().enumerate() {
            out[i] = Self::zigzag_encode(value);
        }
        
        Ok(())
    }
    
    /// Try to decode a slice of unsigned integers back to signed integers, returning
    /// a Result instead of panicking if the output buffer is too small
    /// 
    /// # Arguments
    /// * `values` - Slice of unsigned integers to decode
    /// * `out` - Output slice to store decoded signed integers
    /// 
    /// # Returns
    /// * `Ok(())` if all values were decoded successfully
    /// * `Err(ZigZagError::BufferTooSmall)` if output buffer is too small
    fn try_zigzag_decode_slice(values: &[Self::UInt], out: &mut [Self]) -> Result<(), ZigZagError> 
    where 
        Self: Sized + Copy,
        Self::UInt: Copy
    {
        if out.len() < values.len() {
            return Err(ZigZagError::BufferTooSmall { 
                needed: values.len(), 
                actual: out.len(),
            });
        }
        
        for (i, &value) in values.iter().enumerate() {
            out[i] = Self::zigzag_decode(value);
        }
        
        Ok(())
    }
}

/// Creates an iterator that encodes each signed integer from the source iterator.
///
/// This function provides a zero-copy API for ZigZag encoding. The values are encoded
/// on-the-fly as the iterator is consumed, without requiring an intermediate buffer.
///
/// # Arguments
/// * `iter` - An iterator that yields references to signed integers
///
/// # Returns
/// An iterator that yields encoded unsigned integers
///
/// # Example
/// ```
/// use zigzag_rs::{ZigZag, zigzag_encode_iter};
///
/// let values = [-10, -1, 0, 1, 10];
/// let encoded_iter = zigzag_encode_iter::<i32, _>(values.iter());
///
/// for (original, encoded) in values.iter().zip(encoded_iter) {
///     assert_eq!(encoded, i32::zigzag_encode(*original));
/// }
/// ```
///
/// # Advanced Example
/// ```
/// use zigzag_rs::{ZigZag, zigzag_encode_iter};
///
/// // Filtering and encoding in one pass
/// let values = [-100, -10, -1, 0, 1, 10, 100];
/// 
/// // Process only positive numbers
/// let positive_encoded: Vec<u32> = values.iter()
///     .filter(|&&v| v > 0)
///     .map(|&v| i32::zigzag_encode(v))
///     .collect();
///     
/// assert_eq!(positive_encoded, vec![2, 20, 200]);
///
/// // Alternative approach using zigzag_encode_iter
/// let positive_encoded2: Vec<u32> = zigzag_encode_iter::<i32, _>(
///     values.iter().filter(|&&v| v > 0)
/// ).collect();
///
/// assert_eq!(positive_encoded2, vec![2, 20, 200]);
/// ```
pub fn zigzag_encode_iter<'a, T, I>(iter: I) -> impl Iterator<Item = T::UInt> + 'a
where
    T: ZigZag + Copy + 'a,
    I: Iterator<Item = &'a T> + 'a,
{
    iter.map(|&value| T::zigzag_encode(value))
}

/// Creates an iterator that decodes each unsigned integer from the source iterator.
///
/// This function provides a zero-copy API for ZigZag decoding. The values are decoded
/// on-the-fly as the iterator is consumed, without requiring an intermediate buffer.
///
/// # Arguments
/// * `iter` - An iterator that yields references to unsigned integers
///
/// # Returns
/// An iterator that yields decoded signed integers
///
/// # Example
/// ```
/// use zigzag_rs::{ZigZag, zigzag_decode_iter};
///
/// let encoded = [1u32, 0, 2, 3, 20];
/// let decoded_iter = zigzag_decode_iter::<i32, _>(encoded.iter());
///
/// let expected = [-1, 0, 1, -2, 10];
/// for (expected, decoded) in expected.iter().zip(decoded_iter) {
///     assert_eq!(*expected, decoded);
/// }
/// ```
///
/// # Chaining Example
/// ```
/// use zigzag_rs::{ZigZag, zigzag_encode_iter, zigzag_decode_iter};
///
/// // Encode, then immediately decode without creating intermediate storage
/// let values = [-10, -1, 0, 1, 10];
/// let encoded: Vec<u32> = zigzag_encode_iter::<i32, _>(values.iter()).collect();
/// 
/// // We can decode directly from the encoded values
/// let decoded: Vec<i32> = zigzag_decode_iter::<i32, _>(encoded.iter()).collect();
/// 
/// // Verify values are preserved
/// assert_eq!(values.to_vec(), decoded);
/// ```
pub fn zigzag_decode_iter<'a, T, I>(iter: I) -> impl Iterator<Item = T> + 'a
where
    T: ZigZag + Copy + 'a,
    I: Iterator<Item = &'a T::UInt> + 'a,
    T::UInt: Copy + 'a,
{
    iter.map(|&value| T::zigzag_decode(value))
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
                // Optimized version: combine right shift, negation and XOR in one expression
                ((value >> 1) as Self) ^ (-((value & 1) as Self))
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
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[cfg(test)]
    use std::vec::Vec;
    
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
    fn test_encode_decode_slice_i32() {
        let values = [-100i32, -10, -1, 0, 1, 10, 100];
        let mut encoded = [0u32; 7];
        let mut decoded = [0i32; 7];
        
        // Test encoding slice
        i32::zigzag_encode_slice(&values, &mut encoded);
        assert_eq!(encoded[0], i32::zigzag_encode(-100));
        assert_eq!(encoded[3], i32::zigzag_encode(0));
        assert_eq!(encoded[6], i32::zigzag_encode(100));
        
        // Test decoding slice
        i32::zigzag_decode_slice(&encoded, &mut decoded);
        assert_eq!(values, decoded);
    }
    
    #[test]
    fn test_try_encode_decode_slice() {
        let values = [-100i32, -10, -1, 0, 1, 10, 100];
        
        // Test with correct buffer size
        let mut encoded = [0u32; 7];
        let result = i32::try_zigzag_encode_slice(&values, &mut encoded);
        assert!(result.is_ok());
        
        let mut decoded = [0i32; 7];
        let result = i32::try_zigzag_decode_slice(&encoded, &mut decoded);
        assert!(result.is_ok());
        assert_eq!(values, decoded);
        
        // Test with too small buffer
        let mut small_encoded = [0u32; 3];
        let result = i32::try_zigzag_encode_slice(&values, &mut small_encoded);
        assert!(result.is_err());
        if let Err(ZigZagError::BufferTooSmall { needed, actual }) = result {
            assert_eq!(needed, 7);
            assert_eq!(actual, 3);
        } else {
            panic!("Expected BufferTooSmall error");
        }
        
        let mut small_decoded = [0i32; 3];
        let result = i32::try_zigzag_decode_slice(&encoded, &mut small_decoded);
        assert!(result.is_err());
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
    
    #[test]
    fn test_encode_decode_slice_all_types() {
        // Test i8
        let i8_values = [-100i8, -10, -1, 0, 1, 10, 100];
        let mut i8_encoded = [0u8; 7];
        let mut i8_decoded = [0i8; 7];
        i8::zigzag_encode_slice(&i8_values, &mut i8_encoded);
        i8::zigzag_decode_slice(&i8_encoded, &mut i8_decoded);
        assert_eq!(i8_values, i8_decoded);
        
        // Test i16
        let i16_values = [-1000i16, -100, -10, 0, 10, 100, 1000];
        let mut i16_encoded = [0u16; 7];
        let mut i16_decoded = [0i16; 7];
        i16::zigzag_encode_slice(&i16_values, &mut i16_encoded);
        i16::zigzag_decode_slice(&i16_encoded, &mut i16_decoded);
        assert_eq!(i16_values, i16_decoded);
        
        // Test i64
        let i64_values = [-1000000i64, -10000, -100, 0, 100, 10000, 1000000];
        let mut i64_encoded = [0u64; 7];
        let mut i64_decoded = [0i64; 7];
        i64::zigzag_encode_slice(&i64_values, &mut i64_encoded);
        i64::zigzag_decode_slice(&i64_encoded, &mut i64_decoded);
        assert_eq!(i64_values, i64_decoded);
    }
    
    #[test]
    fn test_zigzag_error() {
        // Just test that we can create the error type and access its fields
        let error = ZigZagError::BufferTooSmall { needed: 10, actual: 5 };
        assert_eq!(error.needed(), 10);
        assert_eq!(error.actual(), 5);
    }
    
    #[test]
    fn test_zigzag_encode_iter() {
        let values = [-100i32, -10, -1, 0, 1, 10, 100];
        
        // Convert to a Vec to compare
        let expected: Vec<u32> = values.iter()
            .map(|&v| i32::zigzag_encode(v))
            .collect();
        
        // Use the zero-copy iterator
        let encoded: Vec<u32> = zigzag_encode_iter::<i32, _>(values.iter()).collect();
        
        assert_eq!(encoded, expected);
        
        // Test with different integer types
        let i8_values = [-100i8, -10, -1, 0, 1, 10, 100];
        let i8_encoded: Vec<u8> = zigzag_encode_iter::<i8, _>(i8_values.iter()).collect();
        
        for (i, &val) in i8_values.iter().enumerate() {
            assert_eq!(i8_encoded[i], i8::zigzag_encode(val));
        }
    }
    
    #[test]
    fn test_zigzag_decode_iter() {
        let encoded = [199u32, 19, 1, 0, 2, 20, 200];
        let expected = [-100i32, -10, -1, 0, 1, 10, 100];
        
        // Use the zero-copy iterator
        let decoded: Vec<i32> = zigzag_decode_iter::<i32, _>(encoded.iter()).collect();
        
        assert_eq!(decoded, expected);
        
        // Test with different integer types
        let i16_encoded = [1999u16, 199, 19, 1, 0, 2, 20, 200, 2000];
        let i16_expected = [-1000i16, -100, -10, -1, 0, 1, 10, 100, 1000];
        
        let i16_decoded: Vec<i16> = zigzag_decode_iter::<i16, _>(i16_encoded.iter()).collect();
        assert_eq!(i16_decoded, i16_expected);
    }
    
    #[test]
    fn test_zero_copy_round_trip() {
        let original = [-1000i16, -100, -10, -1, 0, 1, 10, 100, 1000];
        
        // Encode using iterator
        let encoded: Vec<u16> = zigzag_encode_iter::<i16, _>(original.iter()).collect();
        
        // Decode using iterator
        let decoded: Vec<i16> = zigzag_decode_iter::<i16, _>(encoded.iter()).collect();
        
        // Verify round-trip
        assert_eq!(original.to_vec(), decoded);
    }
}

// Add methods to ZigZagError to access fields without requiring std
impl ZigZagError {
    /// Get the needed buffer size
    pub fn needed(&self) -> usize {
        match self {
            ZigZagError::BufferTooSmall { needed, .. } => *needed,
        }
    }
    
    /// Get the actual buffer size
    pub fn actual(&self) -> usize {
        match self {
            ZigZagError::BufferTooSmall { actual, .. } => *actual,
        }
    }
}
