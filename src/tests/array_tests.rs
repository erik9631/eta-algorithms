use crate::data_structs::array::Array;

#[test]
pub fn array_init_test(){
    let array = Array::<i32>::new(10);
    assert_eq!(array.capacity(), 10);
}

#[test]
pub fn array_default_test(){
    let array = Array::<i8>::new_default_bytes(100, 0);
    assert_eq!(array.capacity(), 100);
    for i in array.iter() {
        assert_eq!(*i, 0i8);
    }
    let array2 = Array::<i8>::new_default_bytes(100, 122);
    for i in array2.iter() {
        assert_eq!(*i, 122i8);
    }
}

#[test]
pub fn array_index_test(){
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    for i in 0..10 {
        assert_eq!(array[i], i as i32);
    }
}

#[test]
pub fn iter_test(){
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    for (idx, item) in array.iter().enumerate() {
        assert_eq!(item, &array[idx]);
    }
}

#[test]
pub fn extend_test(){
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    array.extend(20);
    for i in 0..10 {
        assert_eq!(array[i], i as i32);
    }

    for i in 10..20 {
        array[i] = i as i32;
    }

    for i in 10..20 {
        assert_eq!(array[i], i as i32);
    }
}

#[test]
pub fn extend_by_test(){
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    array.extend_by(10);
    for i in 0..10 {
        assert_eq!(array[i], i as i32);
    }

    for i in 10..20 {
        array[i] = i as i32;
    }

    for i in 10..20 {
        assert_eq!(array[i], i as i32);
    }
}

#[test]
pub fn empty_array_test() {
    let array = Array::<i32>::new(0);
    assert_eq!(array.capacity(), 0);
}

#[test]
#[should_panic]
pub fn out_of_bounds_test() {
    let array = Array::<i32>::new(5);
    let _ = array[5]; // This should panic
}

#[test]
pub fn iter_mut_test() {
    let mut array = Array::<i32>::new(5);
    for i in 0..5 {
        array[i] = i as i32;
    }

    for item in array.iter_mut() {
        *item += 1;
    }

    for (idx, item) in array.iter().enumerate() {
        assert_eq!(*item, (idx as i32) + 1);
    }
}

#[test]
pub fn multiple_extension_test() {
    let mut array = Array::<i32>::new(5);
    for i in 0..5 {
        array[i] = i as i32;
    }
    array.extend_by(5);
    array.extend_by(5);

    for i in 5..15 {
        array[i] = i as i32;
    }

    for i in 0..15 {
        assert_eq!(array[i], i as i32);
    }
    assert_eq!(array.capacity(), 15);
}