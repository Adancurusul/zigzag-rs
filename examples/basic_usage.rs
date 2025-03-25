use zigzag_rs::ZigZag;

fn main() {
    println!("ZigZag Encoding Examples:");
    
    // Encode different integer types (single values)
    println!("Single Value Encoding:");
    println!("i8: -1 encoded as: {} (u8)", i8::zigzag_encode(-1));
    println!("i16: 10 encoded as: {} (u16)", i16::zigzag_encode(10));
    println!("i32: -100 encoded as: {} (u32)", i32::zigzag_encode(-100));
    println!("i64: 1000 encoded as: {} (u64)", i64::zigzag_encode(1000));
    
    println!("\nZigZag Decoding Examples:");
    
    // Decode different integer types (single values)
    println!("Single Value Decoding:");
    println!("u8: 1 decoded as: {} (i8)", i8::zigzag_decode(1u8));
    println!("u16: 20 decoded as: {} (i16)", i16::zigzag_decode(20u16));
    println!("u32: 200 decoded as: {} (i32)", i32::zigzag_decode(200u32));
    println!("u64: 2000 decoded as: {} (i64)", i64::zigzag_decode(2000u64));
    
    // Demonstrate round-trip conversion (single value)
    let original = -42i32;
    let encoded = i32::zigzag_encode(original);
    let decoded = i32::zigzag_decode(encoded);
    
    println!("\nRound-trip Conversion Example (Single Value):");
    println!("Original value: {}", original);
    println!("Encoded as: {}", encoded);
    println!("Decoded back: {}", decoded);
    println!("Conversion successful: {}", original == decoded);
    
    // Demonstrate batch processing
    println!("\nBatch Processing Example:");
    let values = [-100i32, -10, -1, 0, 1, 10, 100];
    let mut encoded = [0u32; 7];
    let mut decoded = [0i32; 7];
    
    // Encode array
    i32::zigzag_encode_slice(&values, &mut encoded);
    
    println!("Original values: {:?}", values);
    println!("Encoded values: {:?}", encoded);
    
    // Decode array
    i32::zigzag_decode_slice(&encoded, &mut decoded);
    println!("Decoded values: {:?}", decoded);
    println!("Batch round-trip successful: {}", values == decoded);
} 