use zigzag_rs::{ZigZag, zigzag_encode_iter, zigzag_decode_iter};

fn main() {
    println!("ZigZag Zero-Copy API Examples");
    println!("=============================\n");
    
    // Source data
    let values = [-100, -10, -1, 0, 1, 10, 100];
    println!("Original values: {:?}", values);
    
    // Basic encode iterator example
    println!("\n1. Basic encoding with iterator");
    println!("----------------------------");
    let encoded_iter = zigzag_encode_iter::<i32, _>(values.iter());
    
    // We can display the values as they are generated
    println!("Encoded values: ");
    for encoded in encoded_iter {
        print!("{} ", encoded);
    }
    println!();
    
    // Demonstrate conversion without allocating a buffer
    println!("\n2. Pairing original and encoded values");
    println!("----------------------------------");
    let encoded_iter = zigzag_encode_iter::<i32, _>(values.iter());
    for (original, encoded) in values.iter().zip(encoded_iter) {
        println!("{:5} encodes to {:5}", original, encoded);
    }
    
    // Decoding example
    println!("\n3. Zero-copy decoding");
    println!("------------------");
    let encoded = [199u32, 19, 1, 0, 2, 20, 200];
    println!("Encoded values: {:?}", encoded);
    
    println!("Decoded values: ");
    let decoded_iter = zigzag_decode_iter::<i32, _>(encoded.iter());
    for decoded in decoded_iter {
        print!("{} ", decoded);
    }
    println!();
    
    // Filtering example
    println!("\n4. Combining with other iterator operations");
    println!("---------------------------------------");
    let values = [-100, -50, -10, -5, -1, 0, 1, 5, 10, 50, 100];
    
    // Create an iterator that only processes positive values
    println!("Original values: {:?}", values);
    println!("Processing only positive values:");
    
    let filtered_encoded: Vec<u32> = values.iter()
        .filter(|&&x| x > 0)  // Only positive values
        .map(|&x| i32::zigzag_encode(x))  // Encode them
        .collect();  // Collect results
    
    println!("Filtered and encoded: {:?}", filtered_encoded);
    
    // Memory efficiency example
    println!("\n5. Memory efficiency comparison");
    println!("----------------------------");
    
    // Traditional approach with intermediate allocations
    let large_range: Vec<i32> = (-1000..1000).collect();
    println!("Processing {} values", large_range.len());
    
    println!("Traditional approach (with buffer allocation):");
    let start = std::time::Instant::now();
    let mut encoded_buffer = vec![0u32; large_range.len()];
    i32::zigzag_encode_slice(&large_range, &mut encoded_buffer);
    
    let mut decoded_buffer = vec![0i32; encoded_buffer.len()];
    i32::zigzag_decode_slice(&encoded_buffer, &mut decoded_buffer);
    let duration = start.elapsed();
    println!("Time with buffers: {:?}", duration);
    
    // Zero-copy approach
    println!("Zero-copy approach (iterator-based):");
    let start = std::time::Instant::now();
    
    // Process without intermediate allocations
    let sum: i32 = zigzag_decode_iter::<i32, _>(
        zigzag_encode_iter::<i32, _>(large_range.iter()).collect::<Vec<_>>().iter()
    ).sum();
    
    let duration = start.elapsed();
    println!("Time with iterators: {:?}", duration);
    println!("Sum of processed values: {}", sum);
} 