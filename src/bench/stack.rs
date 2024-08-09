use crate::data_structs::stack::Stack;
use std::time::Instant;

#[test]
pub fn stack_benchmark(){
    let data_size = 10000000;
    let mut stack = Stack::<i32>::new(2);
    let mut current_size = 2;
    let start = Instant::now();
    for i in 0..data_size {
        if stack.len() == current_size {
            current_size *= 2;
            stack.extend(current_size);
        }
        stack.push(i);
    }

    for i in 0..data_size {
        stack.pop();
    }
    println!("Time taken stack: {:?}", start.elapsed());

    let mut vec_stack = Vec::<i32>::with_capacity(2);
    let start = Instant::now();
    for i in 0..data_size {
        vec_stack.push(i);
    }

    for i in 0..data_size {
        vec_stack.pop();
    }
    println!("Time taken vec_stack: {:?}", start.elapsed());
}