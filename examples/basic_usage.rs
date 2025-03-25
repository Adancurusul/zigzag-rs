use zigzag_rs::ZigZag;

fn main() {
    println!("ZigZag Encoding Examples:");
    
    // Encode different integer types
    println!("i8: -1 encoded as: {} (u8)", i8::zigzag_encode(-1));
    println!("i16: 10 encoded as: {} (u16)", i16::zigzag_encode(10));
    println!("i32: -100 encoded as: {} (u32)", i32::zigzag_encode(-100));
    println!("i64: 1000 encoded as: {} (u64)", i64::zigzag_encode(1000));
    
    println!("\nZigZag Decoding Examples:");
    
    // Decode different integer types
    println!("u8: 1 decoded as: {} (i8)", i8::zigzag_decode(1u8));
    println!("u16: 20 decoded as: {} (i16)", i16::zigzag_decode(20u16));
    println!("u32: 200 decoded as: {} (i32)", i32::zigzag_decode(200u32));
    println!("u64: 2000 decoded as: {} (i64)", i64::zigzag_decode(2000u64));
    
    // Demonstrate round-trip conversion
    let original = -42i32;
    let encoded = i32::zigzag_encode(original);
    let decoded = i32::zigzag_decode(encoded);
    
    println!("\nRound-trip Conversion Example:");
    println!("Original value: {}", original);
    println!("Encoded as: {}", encoded);
    println!("Decoded back: {}", decoded);
    println!("Conversion successful: {}", original == decoded);
} 