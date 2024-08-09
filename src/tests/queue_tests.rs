use crate::data_structs::queue::Queue;

#[test]
pub fn test_queue_init(){
    let mut queue = Queue::<i32>::new_pow2_sized(10);
    assert_eq!(queue.capacity(), 10);
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
pub fn test_queue_dequeue_cyclic() {
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
pub fn test_queue_extend(){
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