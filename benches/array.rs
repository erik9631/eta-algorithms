use std::time::Instant;
use criterion::{black_box, Criterion, criterion_group, criterion_main};
use eta_algorithms::data_structs::array::Array;

fn benchmark_array(initial_size: usize, data_size: usize){
    // Benchmark custom Array
    let mut custom_array = Array::new(initial_size);
    let mut custom_len = 0;
    let mut current_size = initial_size;

    // Push
    for i in 0..data_size {
        if custom_len == current_size {
            current_size *= 2;
            custom_array.extend(current_size);
        }
        custom_array[custom_len] = i;
        custom_len += 1;
    }

    // Sum (simulating pop)
    let mut custom_sum = 0;
    for i in 0..data_size {
        custom_sum += custom_array[i];
    }
    black_box(custom_sum);
}

fn benchmark_vec(initial_size: usize, data_size: usize) {

    // Benchmark Vec
    let mut vec = Vec::with_capacity(initial_size);
    let start = Instant::now();

    // Push
    for i in 0..data_size {
        if vec.len() == vec.capacity() {
            vec.reserve(vec.capacity());
        }
        vec.push(i);
    }
    let vec_sum: usize = vec.iter().sum();
    black_box(vec_sum);
}

fn array_vs_vec(c: &mut Criterion) {
    const INITIAL_SIZE: usize = 2;
    const DATA_SIZE: usize = 100_000_000;
    let mut group = c.benchmark_group("Array vs Vec");
    group.bench_function("Array", |b| b.iter(|| benchmark_array(INITIAL_SIZE, DATA_SIZE)));
    group.bench_function("Vec", |b| b.iter(|| benchmark_vec(INITIAL_SIZE, DATA_SIZE)));
    group.finish();
}

criterion_group!{
    name=array;
    config = Criterion::default().sample_size(50);
    targets = array_vs_vec
}
criterion_main!(array);