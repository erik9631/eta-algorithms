use std::collections::VecDeque;
use criterion::{black_box, Criterion, criterion_group, criterion_main};
use eta_algorithms::data_structs::queue::Queue;

fn custom_queue_benchmark(num_elements: usize){
    // Benchmark custom Queue
    let mut custom_queue = Queue::new_pow2_sized(num_elements);

    // Push
    for i in 0..num_elements {
        custom_queue.push(i);
    }

    let mut custom_sum = 0;
    for _ in 0..num_elements {
        custom_sum += custom_queue.dequeue().unwrap();
    }
    black_box(custom_sum);
}

fn vecdeque_benchmark(num_elements: usize){
    // Benchmark VecDeque
    let mut vec_deque = VecDeque::with_capacity(num_elements);

    // Push
    for i in 0..num_elements {
        vec_deque.push_back(i);
    }

    // Pop
    let mut vecdeque_sum = 0;
    for _ in 0..num_elements {
        vecdeque_sum += vec_deque.pop_front().unwrap();
    }

    black_box(vecdeque_sum);
}

fn benchmark_queue_vs_vecdeque(c: &mut Criterion) {
    const NUM_ELEMENTS: usize = 10_000_000;
    let mut group = c.benchmark_group("Queue vs VecDeque");
    group.bench_function("Custom Queue", |b| b.iter(|| custom_queue_benchmark(NUM_ELEMENTS)));
    group.bench_function("VecDeque", |b| b.iter(|| vecdeque_benchmark(NUM_ELEMENTS)));
    group.finish();
}

criterion_group!{
    name=queue;
    config = Criterion::default().sample_size(50);
    targets = benchmark_queue_vs_vecdeque
}
criterion_main!(queue);