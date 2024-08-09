use crate::data_structs::queue::Queue;

#[test]
pub fn test_queue_init(){
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    assert_eq!(queue.capacity(), 16);
    assert_eq!(queue.len(), 0);
}

#[test]
pub fn test_queue_push_pop(){
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    for i in 0..10 {
        queue.push(i);
        assert_eq!(queue.len() as i32, i + 1);
    }

    for i in 0..10 {
        assert_eq!(queue.dequeue(), Some(i));
        assert_eq!(queue.len() as i32, 9 - i);
    }
}

#[test]
#[should_panic]
pub fn test_queue_push_over_capacity(){
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    for i in 0..17 {
        queue.push(i);
        assert_eq!(queue.len() as i32, i + 1);
    }
}

#[test]
pub fn test_queue_front(){
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    for i in 0..10 {
        queue.push(i);
    }
    assert_eq!(queue.front(), Some(&0));
}

#[test]
pub fn test_queue_dequeue_wrap() {
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    for i in 0..16 {
        queue.push(i);
    }
    for i in 0..4 {
        assert_eq!(queue.dequeue(), Some(i));
    }

    for i in 20..24 {
        queue.push(i);
    }

    for i in 4..8 {
        assert_eq!(queue.dequeue(), Some(i));
    }

    for i in 24..28 {
        queue.push(i);
    }

    for i in 8 .. 16{
        assert_eq!(queue.dequeue(), Some(i));
    }

    for i in 20.. 24{
        assert_eq!(queue.dequeue(), Some(i));
    }

    for i in 24 .. 28{
        assert_eq!(queue.dequeue(), Some(i));
    }
}

#[test]
pub fn test_queue_front_mut() {
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    for i in 0..10 {
        queue.push(i);
    }
    *queue.front_mut().unwrap() = 100;
    assert_eq!(queue.front(), Some(&100));
}

#[test]
pub fn dequeue_empty_test(){
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    assert_eq!(queue.dequeue(), None);
}

#[test]
pub fn test_queue_front_empty(){
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    assert_eq!(queue.front(), None);
}

#[test]
pub fn test_queue_extend_full(){
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    for i in 0..16 {
        queue.push(i);
    }
    queue.extend_pow2_sized(20);
    assert_eq!(queue.capacity(), 32);
    for i in 0..16 {
        assert_eq!(queue.dequeue(), Some(i));
    }

    for i in 0..32 {
        queue.push(i);
        assert_eq!(queue.len() as i32, i + 1);
    }

    for i in 0..32 {
        assert_eq!(queue.dequeue(), Some(i));
    };
}

#[test]
pub fn test_queue_extend_half() {
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    for i in 0..8 {
        queue.push(i);
    }
    queue.extend_pow2_sized(20);
    assert_eq!(queue.capacity(), 32);
    for i in 8..16 {
        queue.push(i);
    }
    for i in 0..16 {
        assert_eq!(queue.dequeue(), Some(i));
    }
}

#[test]
pub fn test_queue_extend_wrap1() {
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    for i in 0..16 {
        queue.push(i);
    }
    for i in 0..4 {
        assert_eq!(queue.dequeue(), Some(i));
    }

    for i in 16..20 {
        queue.push(i);
    }

    queue.extend_pow2_sized(32);
    assert_eq!(queue.capacity(), 32);

    for i in 20..32{
        queue.push(i);
    }

    for i in  4..32 {
        assert_eq!(queue.dequeue(), Some(i));
    }
}

#[test]
pub fn test_push_pop_wrap2() {
    let mut queue = Queue::<i32>::new_pow2_sized(4);
    for i in 0..3 {
        queue.push(i);
    }
    assert_eq!(queue.dequeue(), Some(0));
    assert_eq!(queue.dequeue(), Some(1));
    queue.push(3);
    queue.push(4);
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), Some(3));
    assert_eq!(queue.dequeue(), Some(4));
    assert_eq!(queue.dequeue(), None);
}

#[test]
pub fn test_extend_empty_queue() {
    let mut queue = Queue::<i32>::new_pow2_sized(4);
    queue.extend_pow2_sized(8);
    assert_eq!(queue.capacity(), 8);
    assert_eq!(queue.len(), 0);
    for i in 0..8 {
        queue.push(i);
    }
    for i in 0..8 {
        assert_eq!(queue.dequeue(), Some(i));
    }
}

#[test]
pub fn test_push_pop_at_capacity_boundary() {
    let mut queue = Queue::<i32>::new_pow2_sized(4);
    for i in 0..4 {
        queue.push(i);
    }
    assert_eq!(queue.dequeue(), Some(0));
    queue.push(4);
    for i in 1..5 {
        assert_eq!(queue.dequeue(), Some(i));
    }
}

#[test]
pub fn test_extend_almost_full_queue() {
    let mut queue = Queue::<i32>::new_pow2_sized(4);
    for i in 0..3 {
        queue.push(i);
    }
    queue.extend_pow2_sized(8);
    assert_eq!(queue.capacity(), 8);
    for i in 3..8 {
        queue.push(i);
    }
    for i in 0..8 {
        assert_eq!(queue.dequeue(), Some(i));
    }
}

#[test]
pub fn test_multiple_extensions() {
    let mut queue = Queue::<i32>::new_pow2_sized(2);
    queue.extend_pow2_sized(4);
    queue.extend_pow2_sized(8);
    queue.extend_pow2_sized(16);
    assert_eq!(queue.capacity(), 16);
    for i in 0..16 {
        queue.push(i);
    }
    for i in 0..16 {
        assert_eq!(queue.dequeue(), Some(i));
    }
}

#[test]
pub fn test_extend_wrap_dequeue() {
    let mut queue = Queue::<i32>::new_pow2_sized(4);
    for i in 0..4 {
        queue.push(i);
    }
    assert_eq!(queue.dequeue(), Some(0));
    assert_eq!(queue.dequeue(), Some(1));
    queue.push(4);
    queue.push(5);
    queue.extend_pow2_sized(8);
    assert_eq!(queue.capacity(), 8);
    for i in 2..6 {
        assert_eq!(queue.dequeue(), Some(i));
    }
    assert_eq!(queue.dequeue(), None);
}

#[test]
pub fn test_alternating_push_pop() {
    let mut queue = Queue::<i32>::new_pow2_sized(4);

    queue.push(1);
    queue.push(2);
    assert_eq!(queue.dequeue(), Some(1));

    queue.push(3);
    assert_eq!(queue.dequeue(), Some(2));

    queue.push(4);
    queue.push(5);
    assert_eq!(queue.dequeue(), Some(3));

    queue.push(6);
    assert_eq!(queue.dequeue(), Some(4));

    assert_eq!(queue.len(), 2);
    assert_eq!(queue.dequeue(), Some(5));
    assert_eq!(queue.dequeue(), Some(6));
    assert_eq!(queue.dequeue(), None);

    assert_eq!(queue.len(), 0);
}