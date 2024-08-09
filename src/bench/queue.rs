use std::collections::VecDeque;
use std::time::Instant;
use crate::data_structs::queue::Queue;

#[test]
fn benchmark_queue_vs_vecdeque() {
    const NUM_ELEMENTS: usize = 10_000_000;

    // Benchmark custom Queue
    let mut custom_queue = Queue::new_pow2_sized(NUM_ELEMENTS);
    let start = Instant::now();

    // Push
    for i in 0..NUM_ELEMENTS {
        custom_queue.push(i);
    }
    let custom_push_time = start.elapsed();

    // Pop
    let start = Instant::now();
    let mut custom_sum = 0;
    for _ in 0..NUM_ELEMENTS {
        custom_sum += custom_queue.dequeue().unwrap();
    }
    let custom_pop_time = start.elapsed();

    // Benchmark VecDeque
    let mut vec_deque = VecDeque::with_capacity(NUM_ELEMENTS);
    let start = Instant::now();

    // Push
    for i in 0..NUM_ELEMENTS {
        vec_deque.push_back(i);
    }
    let vecdeque_push_time = start.elapsed();

    // Pop
    let start = Instant::now();
    let mut vecdeque_sum = 0;
    for _ in 0..NUM_ELEMENTS {
        vecdeque_sum += vec_deque.pop_front().unwrap();
    }
    let vecdeque_pop_time = start.elapsed();

    println!("Custom Queue:");
    println!("  Push time: {:?}", custom_push_time);
    println!("  Pop time:  {:?}", custom_pop_time);
    println!("VecDeque:");
    println!("  Push time: {:?}", vecdeque_push_time);
    println!("  Pop time:  {:?}", vecdeque_pop_time);

    // Prevent optimizations by using the sums
    println!("Custom sum: {}", custom_sum);
    println!("VecDeque sum: {}", vecdeque_sum);

    // Optional: Assert that custom Queue is faster (you may need to adjust these)
    assert!(custom_push_time < vecdeque_push_time, "Custom Queue push should be faster");
    assert!(custom_pop_time < vecdeque_pop_time, "Custom Queue pop should be faster");
}