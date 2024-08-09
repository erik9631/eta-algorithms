use std::time::Instant;
use crate::data_structs::array::Array; // Make sure to import your Array type

#[test]
fn benchmark_array_vs_vec() {
    const INITIAL_SIZE: usize = 2;
    const DATA_SIZE: usize = 100_000_000;

    // Benchmark custom Array
    let mut custom_array = Array::new(INITIAL_SIZE);
    let mut custom_len = 0;
    let mut current_size = INITIAL_SIZE;
    let start = Instant::now();

    // Push
    for i in 0..DATA_SIZE {
        if custom_len == current_size {
            current_size *= 2;
            custom_array.extend(current_size);
        }
        custom_array[custom_len] = i;
        custom_len += 1;
    }
    let custom_push_time = start.elapsed();

    // Sum (simulating pop)
    let start = Instant::now();
    let mut custom_sum = 0;
    for i in 0..DATA_SIZE {
        custom_sum += custom_array[i];
    }
    let custom_sum_time = start.elapsed();

    // Benchmark Vec
    let mut vec = Vec::with_capacity(INITIAL_SIZE);
    let start = Instant::now();

    // Push
    for i in 0..DATA_SIZE {
        if vec.len() == vec.capacity() {
            vec.reserve(vec.capacity());
        }
        vec.push(i);
    }
    let vec_push_time = start.elapsed();

    // Sum (simulating pop)
    let start = Instant::now();
    let vec_sum: usize = vec.iter().sum();
    let vec_sum_time = start.elapsed();

    println!("Custom Array:");
    println!("  Push time: {:?}", custom_push_time);
    println!("  Sum time:  {:?}", custom_sum_time);
    println!("Vec:");
    println!("  Push time: {:?}", vec_push_time);
    println!("  Sum time:  {:?}", vec_sum_time);

    // Prevent optimizations by using the sums
    println!("Custom sum: {}", custom_sum);
    println!("Vec sum: {}", vec_sum);
}