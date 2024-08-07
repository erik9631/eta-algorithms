use crate::data_structs::array::Array;

#[test]
pub fn array_init_test(){
    let array = Array::<i32>::new(10);
    assert_eq!(array.capacity(), 10);
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