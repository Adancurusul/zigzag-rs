use zigzag_rs::ZigZag;

// Since we don't depend on std, we use a simple method to measure performance
fn main() {
    const ITERATIONS: usize = 10_000_000;
    const BATCH_SIZE: usize = 1000;
    
    println!("Running ZigZag encoding/decoding performance test");
    
    // Test single value i32 encoding
    let start = std::time::Instant::now();
    let mut sum = 0u32;
    for i in 0..ITERATIONS {
        let val = (i % 1000) as i32 - 500; // Cycle between -500 and 499
        sum = sum.wrapping_add(i32::zigzag_encode(val));
    }
    let duration = start.elapsed();
    println!("i32 single value encoding: {:?} for {} operations (checksum: {})", 
             duration, ITERATIONS, sum);
    
    // Test single value i32 decoding
    let start = std::time::Instant::now();
    let mut sum = 0i32;
    for i in 0..ITERATIONS {
        let val = (i % 1000) as u32;
        sum = sum.wrapping_add(i32::zigzag_decode(val));
    }
    let duration = start.elapsed();
    println!("i32 single value decoding: {:?} for {} operations (checksum: {})", 
             duration, ITERATIONS, sum);
    
    // Test batch i32 encoding
    let mut values = [0i32; BATCH_SIZE];
    let mut encoded = [0u32; BATCH_SIZE];
    
    // Initialize test data
    for i in 0..BATCH_SIZE {
        values[i] = (i as i32) - (BATCH_SIZE as i32 / 2);
    }
    
    // Measure batch encoding performance
    let batch_iterations = ITERATIONS / BATCH_SIZE;
    let start = std::time::Instant::now();
    for _ in 0..batch_iterations {
        i32::zigzag_encode_slice(&values, &mut encoded);
    }
    let duration = start.elapsed();
    println!("i32 batch encoding: {:?} for {} batches of {} values (total: {} operations)", 
             duration, batch_iterations, BATCH_SIZE, batch_iterations * BATCH_SIZE);
    
    // Measure batch decoding performance
    let start = std::time::Instant::now();
    for _ in 0..batch_iterations {
        i32::zigzag_decode_slice(&encoded, &mut values);
    }
    let duration = start.elapsed();
    println!("i32 batch decoding: {:?} for {} batches of {} values (total: {} operations)", 
             duration, batch_iterations, BATCH_SIZE, batch_iterations * BATCH_SIZE);
    
    // Check if expected values after round-trip are preserved
    let mut original = [0i32; BATCH_SIZE];
    for i in 0..BATCH_SIZE {
        original[i] = (i as i32) - (BATCH_SIZE as i32 / 2);
    }
    assert_eq!(values, original, "Batch round-trip values should be preserved");
    
    println!("\nComparing single value vs batch processing efficiency:");
    println!("This comparison shows if batch processing provides any optimization beyond \
              just calling single operations in sequence.");
} 