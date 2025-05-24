use crate::data_structs::stack::Stack;

#[test]
pub fn stack_init_test() {
    let stack = Stack::<i32>::new(10);
    assert_eq!(stack.capacity(), 10);
}

#[test]
pub fn stack_push_pop_test() {
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
        assert_eq!(stack.len() as i32, i + 1);
    }

    let mut len_counter = 9;
    for i in (0..10).rev() {
        assert_eq!(stack.pop(), Some(i));
        assert_eq!(stack.len() as i32, len_counter);
        len_counter -= 1;
    }
}

#[test]
pub fn stack_top_pop_test() {
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
        assert_eq!(stack.len() as i32, i + 1);
    }

    let mut len_counter = 9;
    for i in (0..10).rev() {
        assert_eq!(stack.top(), Some(&i));
        assert_eq!(stack.pop(), Some(i));
        assert_eq!(stack.len() as i32, len_counter);
        len_counter -= 1;
    }
}

#[test]
pub fn stack_extend_test() {
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }

    stack.extend(20);
    assert_eq!(stack.capacity(), 20);

    for i in 10..20 {
        stack.push(i);
        assert_eq!(stack.len() as i32, i + 1);
    }

    let mut len_counter = 19;
    for i in (0..20).rev() {
        assert_eq!(stack.pop(), Some(i));
        assert_eq!(stack.len() as i32, len_counter);
        len_counter -= 1;
    }
}

#[test]
pub fn stack_extend_by_test() {
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }

    stack.extend_by(10);
    assert_eq!(stack.capacity(), 20);

    for i in 10..20 {
        stack.push(i);
        assert_eq!(stack.len() as i32, i + 1);
    }

    let mut len_counter = 19;
    for i in (0..20).rev() {
        assert_eq!(stack.pop(), Some(i));
        assert_eq!(stack.len() as i32, len_counter);
        len_counter -= 1;
    }
}

#[test]
#[should_panic]
pub fn extend_over_capacity() {
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
        assert_eq!(stack.len() as i32, i + 1);
    }

    stack.extend(10);
    assert_eq!(stack.capacity(), 10);

    for i in 10..20 {
        stack.push(i);
        assert_eq!(stack.len() as i32, i + 1);
    }

    for i in (0..20).rev() {
        assert_eq!(stack.pop(), Some(i));
    }
}

#[test]
pub fn top_mut_test() {
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }

    let top = stack.top_mut().unwrap();
    *top = 100;
    assert_eq!(stack.top(), Some(&100));
}

#[test]
pub fn empty_top_test() {
    let stack = Stack::<i32>::new(10);
    assert_eq!(stack.top(), None);
}

#[test]
#[should_panic]
pub fn over_capacity_test() {
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }
    assert_eq!(stack.capacity(), 10);
    stack.push(100);
}

#[test]
pub fn stack_index_test() {
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
        assert_eq!(stack.len() as i32, i + 1);
    }

    let mut counter = 9;
    for i in 0..10 {
        assert_eq!(stack[-i], counter);
        counter -= 1;
    }
}

#[test]
#[should_panic]
pub fn stack_index_out_of_bounds_test() {
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }
    stack[-10];
}
#[test]
pub fn test_stack_pop_empty() {
    let mut stack = Stack::<i32>::new(10);
    assert_eq!(stack.pop(), None);
}

#[test]
pub fn test_stack_top_empty() {
    let stack = Stack::<i32>::new(10);
    assert_eq!(stack.top(), None);
}
#[test]
pub fn test_stack_multiple_extend() {
    let mut stack = Stack::<i32>::new(5);
    stack.push(1);
    stack.push(2);
    stack.extend(10);
    assert_eq!(stack.capacity(), 10);
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.top(), Some(2).as_ref());
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    stack.extend(15);
    stack.extend(20);
    assert_eq!(stack.capacity(), 20);
}

#[test]
pub fn test_stack_with_non_copy_type() {
    let mut stack = Stack::<&str>::new(5);
    stack.push("Hello");
    stack.push("World");
    assert_eq!(stack.pop(), Some("World"));
    assert_eq!(stack.pop(), Some("Hello"));
    assert_eq!(stack.pop(), None);
}

#[test]
pub fn test_stack_multiple_extend_debug() {
    let mut stack = Stack::<i32>::new(5);
    stack.push(1);
    stack.push(2);
    println!("{:?}", stack);
    stack.extend(10);
    assert_eq!(stack.capacity(), 10);
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.top(), Some(2).as_ref());
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    println!("{:?}", stack);
    stack.extend(15);
    stack.extend(20);
    assert_eq!(stack.capacity(), 20);
    println!("{:?}", stack);
}
