use zigzag_rs::ZigZag;

// Since we don't depend on std, we use a simple method to measure performance
fn main() {
    const ITERATIONS: usize = 10_000_000;
    
    println!("Running ZigZag encoding/decoding performance test ({} iterations)", ITERATIONS);
    
    // Test i32 encoding
    let start = std::time::Instant::now();
    let mut sum = 0u32;
    for i in 0..ITERATIONS {
        let val = (i % 1000) as i32 - 500; // Cycle between -500 and 499
        sum = sum.wrapping_add(i32::zigzag_encode(val));
    }
    let duration = start.elapsed();
    println!("i32 encoding: {:?} (checksum: {})", duration, sum);
    
    // Test i32 decoding
    let start = std::time::Instant::now();
    let mut sum = 0i32;
    for i in 0..ITERATIONS {
        let val = (i % 1000) as u32;
        sum = sum.wrapping_add(i32::zigzag_decode(val));
    }
    let duration = start.elapsed();
    println!("i32 decoding: {:?} (checksum: {})", duration, sum);
    
    // Test i64 encoding
    let start = std::time::Instant::now();
    let mut sum = 0u64;
    for i in 0..ITERATIONS {
        let val = (i % 1000) as i64 - 500;
        sum = sum.wrapping_add(i64::zigzag_encode(val));
    }
    let duration = start.elapsed();
    println!("i64 encoding: {:?} (checksum: {})", duration, sum);
    
    // Test i64 decoding
    let start = std::time::Instant::now();
    let mut sum = 0i64;
    for i in 0..ITERATIONS {
        let val = (i % 1000) as u64;
        sum = sum.wrapping_add(i64::zigzag_decode(val));
    }
    let duration = start.elapsed();
    println!("i64 decoding: {:?} (checksum: {})", duration, sum);
} 