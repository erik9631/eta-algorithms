use crate::data_structs::stack::Stack;

#[test]
pub fn stack_init_test(){
    let stack = Stack::<i32>::new(10);
    assert_eq!(stack.capacity(), 10);
}

#[test]
pub fn stack_push_pop_test(){
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }

    for i in (0..10).rev() {
        assert_eq!(stack.pop(), Some(i));
    }
}

#[test]
pub fn stack_top_pop_test(){
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }

    for i in (0..10).rev() {
        assert_eq!(stack.top(), Some(&i));
        assert_eq!(stack.pop(), Some(i));
    }
}

#[test]
pub fn stack_extend_test(){
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }

    stack.extend(20);

    for i in 10..20 {
        stack.push(i);
    }

    for i in (0..20).rev() {
        assert_eq!(stack.pop(), Some(i));
    }
}

#[test]
pub fn stack_extend_by_test(){
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }

    stack.extend_by(20);

    for i in 10..20 {
        stack.push(i);
    }

    for i in (0..20).rev() {
        assert_eq!(stack.pop(), Some(i));
    }
}


#[test]
#[should_panic]
pub fn extend_over_capacity(){
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }

    stack.extend(10);

    for i in 10..20 {
        stack.push(i);
    }

    for i in (0..20).rev() {
        assert_eq!(stack.pop(), Some(i));
    }

}

#[test]
pub fn top_mut_test(){
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);
    }

    let top = stack.top_mut().unwrap();
    *top = 100;
    assert_eq!(stack.top(), Some(&100));
}

#[test]
pub fn empty_top_test(){
    let mut stack = Stack::<i32>::new(10);
    assert_eq!(stack.top(), None);
}

#[test]
#[should_panic]
pub fn over_capacity_test(){
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);

    }
    assert_eq!(stack.capacity(), 10);
    stack.push(100);
}

#[test]
pub fn stack_index_test(){
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);

    }

    let mut counter = 9;
    for i in 0..10 {
        assert_eq!(stack[-i], counter);
        counter -= 1;
    }
}

#[test]
#[should_panic]
pub fn stack_index_out_of_bounds_test(){
    let mut stack = Stack::<i32>::new(10);
    for i in 0..10 {
        stack.push(i);

    }
    stack[-10];
}