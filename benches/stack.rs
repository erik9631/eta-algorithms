use std::time::Instant;
use criterion::{black_box, Criterion, criterion_group, criterion_main};
use eta_algorithms::data_structs::stack::Stack;

fn stack_benchmark(data_size: usize){
    let mut stack = Stack::<i32>::new(2);
    let mut current_size = 2;
    for i in 0..data_size {
        if stack.len() == current_size {
            current_size *= 2;
            stack.extend(current_size);
        }
        stack.push(i as i32);
    }
    let mut sum = 0;

    for i in 0..data_size {
        sum += stack.pop().unwrap();
    }
    black_box(sum);
}

pub fn vec_stack_benchmark(data_size: usize){
    let mut vec_stack = Vec::<i32>::with_capacity(2);
    for i in 0..data_size {
        vec_stack.push(i as i32);
    }

    let mut sum = 0;
    for i in 0..data_size {
        sum += vec_stack.pop().unwrap();
    }
    black_box(sum);
}

fn stack_vs_vec(criterion: &mut Criterion){
    let data_size = 10000000;
    let mut group = criterion.benchmark_group("Stack vs Vec");
    group.bench_function("Stack", |b| b.iter(|| stack_benchmark(data_size)));
    group.bench_function("Vec", |b| b.iter(|| vec_stack_benchmark(data_size)));
    group.finish();
}

criterion_group!{
    name=stack;
    config = Criterion::default().sample_size(50);
    targets = stack_vs_vec
}
criterion_main!(stack);