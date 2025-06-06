use crate::data_structs::array::Array;

#[test]
pub fn array_init_test() {
    let array = Array::<i32>::new(10);
    assert_eq!(array.capacity(), 10);
}

#[test]
pub fn array_default_test() {
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
pub fn array_index_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    for i in 0..10 {
        assert_eq!(array[i], i as i32);
    }
}

#[test]
pub fn iter_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    for (idx, item) in array.iter().enumerate() {
        assert_eq!(item, &array[idx]);
    }
}

#[test]
pub fn extend_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    array.resize(20);
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
pub fn extend_by_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    array.resize_by(10);
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
    array.resize_by(5);
    array.resize_by(5);

    for i in 5..15 {
        array[i] = i as i32;
    }

    for i in 0..15 {
        assert_eq!(array[i], i as i32);
    }
    assert_eq!(array.capacity(), 15);
}

#[test]
pub fn split_at_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }
    let (left, right) = array.split_at(5);
    for i in 0..5 {
        assert_eq!(left[i], i as i32);
    }

    for i in 5..10 {
        assert_eq!(right[i - 5], i as i32);
    }
}

#[test]
pub fn split_at_mut_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }
    let (left, right) = array.split_at_mut(5);
    for i in 0..5 {
        left[i] += 10;
        assert_eq!(left[i], (i + 10) as i32);
    }

    for i in 5..10 {
        right[i - 5] += 10;
        assert_eq!(right[i - 5], (i + 10) as i32);
    }
}

#[test]
pub fn split_into_parts_test() {
    let mut array = Array::<i32>::new(100);
    for i in 0..100 {
        array[i] = i as i32;
    }
    let parts = array.split_into_parts(5);
    assert_eq!(parts.capacity(), 5);
    for i in 0..5 {
        for j in 0..20 {
            assert_eq!(parts[i][j], (i * 20 + j) as i32);
        }
    }
}

#[test]
pub fn split_into_parts_mut_test() {
    let mut array = Array::<i32>::new(100);
    for i in 0..100 {
        array[i] = i as i32;
    }
    let mut parts = array.split_into_parts_mut(5);
    assert_eq!(parts.capacity(), 5);
    for i in 0..5 {
        for j in 0..20 {
            parts[i][j] = (1000 + i * 20 + j) as i32;
            assert_eq!(parts[i][j], (1000 + i * 20 + j) as i32);
        }
    }
}

#[test]
pub fn split_at_edge_cases() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    // Split at 0
    let (left, right) = array.split_at(0);
    assert_eq!(left.len(), 0);
    assert_eq!(right.len(), 10);

    // Split at last index
    let (left, right) = array.split_at(9);
    assert_eq!(left.len(), 9);
    assert_eq!(right.len(), 1);
}

#[test]
pub fn split_into_parts_uneven() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    let parts = array.split_into_parts(3);
    assert_eq!(parts.capacity(), 3);
    assert_eq!(parts[0].len(), 3);
    assert_eq!(parts[1].len(), 3);
    assert_eq!(parts[2].len(), 4); // Last part gets the remainder
}

#[test]
pub fn split_into_parts_multiple_times() {
    let mut array = Array::<i32>::new(5);
    for i in 0..5 {
        array[i] = i as i32;
    }

    // Split into 1 part
    let parts = array.split_into_parts(1);
    assert_eq!(parts.capacity(), 1);
    assert_eq!(parts[0].len(), 5);

    // Split into 5 parts
    let parts = array.split_into_parts(5);
    assert_eq!(parts.capacity(), 5);
    for i in 0..5 {
        assert_eq!(parts[i].len(), 1);
        assert_eq!(parts[i][0], i as i32);
    }
}

#[test]
pub fn split_into_parts_mut_affects_original() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    {
        let mut parts = array.split_into_parts_mut(2);
        parts[0][0] = 100;
        parts[1][4] = 200;
    }

    assert_eq!(array[0], 100);
    assert_eq!(array[9], 200);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
pub fn split_at_out_of_bounds() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    // This should panic
    array.split_at(11);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
pub fn split_at_mut_out_of_bounds() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    // This should panic
    array.split_at_mut(11);
}

#[test]
#[should_panic(expected = "Parts cannot be 0")]
pub fn split_into_parts_zero() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    // This should panic
    array.split_into_parts(0);
}

#[test]
#[should_panic(expected = "Parts cannot be 0")]
pub fn split_into_parts_mut_zero() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    // This should panic
    array.split_into_parts_mut(0);
}

#[test]
#[should_panic(expected = "Parts must be less than or equal to the capacity of the array")]
pub fn split_into_parts_too_large() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }

    // This should panic
    array.split_into_parts(11);
}

#[test]
pub fn new_with_default_test() {
    let array = Array::<i32>::new_with_default(10, 123456);
    assert_eq!(array.capacity(), 10);
    for i in 0..10 {
        assert_eq!(array[i], 123456);
    }
}

#[test]
pub fn clone_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }
    let array2 = array.clone();
    assert_eq!(array2.capacity(), 10);
    for i in 0..10 {
        assert_eq!(array2[i], i as i32);
    }
}

#[test]
pub fn iter_range_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }
    let iter = array.iter_range(2, 5);
    for (i, item) in iter.enumerate() {
        assert_eq!(*item, i as i32 + 2);
    }
}

#[test]
pub fn iter_range_mut_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }
    let iter = array.iter_range_mut(2, 5);
    for item in iter {
        *item = 100;
    }
    for item in array.iter_range(2, 5) {
        assert_eq!(*item, 100);
    }
}

#[test]
pub fn iter_range_out_of_bounds_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }
    let iter = array.iter_range(2, 11);
    for (i, item) in iter.enumerate() {
        assert_eq!(*item, i as i32 + 2);
    }
}

#[test]
pub fn iter_range_mut_out_of_bounds_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }
    let iter = array.iter_range_mut(2, 13);
    for item in iter {
        *item = 100;
    }
    for item in array.iter_range(2, 13) {
        assert_eq!(*item, 100);
    }
}

#[test]
fn multiple_unchecked_iter_range_mut_test() {
    let mut array = Array::<i32>::new(10);
    for i in 0..10 {
        array[i] = i as i32;
    }
    let iter1 = unsafe { array.iter_range_mut_unchecked(0, 3) };
    let iter2 = unsafe { array.iter_range_mut_unchecked(3, 5) };
    let iter3 = unsafe { array.iter_range_mut_unchecked(5, 10) };

    for item in iter1 {
        *item = 100;
    }
    for item in iter2 {
        *item = 200;
    }
    for item in iter3 {
        *item = 300;
    }

    for item in array.iter_range(0, 2) {
        assert_eq!(*item, 100);
    }
    for item in array.iter_range(3, 5) {
        assert_eq!(*item, 200);
    }
    for item in array.iter_range(6, 9) {
        assert_eq!(*item, 300);
    }
}

#[test]
fn from_vec_test() {
    let vec = vec![1, 2, 3, 4, 5];
    let array = Array::from_vec(vec);
    assert_eq!(array.capacity(), 5);
    for i in 0..5 {
        assert_eq!(array[i], i + 1);
    }
}

#[test]
fn from_vec_test_large() {
    let mut vec = Vec::with_capacity(10000);
    for i in 0..10000 {
        vec.push(i);
    }
    let array = Array::from_vec(vec);
    assert_eq!(array.capacity(), 10000);
    for (idx, i) in array.iter().enumerate() {
        assert_eq!(*i, idx);
    }
}

#[test]
fn from_slice_test() {
    let vec = &[1, 2, 3, 4, 5];
    let array = Array::from_slice(vec.as_slice());
    assert_eq!(array.capacity(), 5);
    for i in 0..5 {
        assert_eq!(array[i], i + 1);
    }
}

#[test]
fn from_slice_test_large() {
    let mut vec = Vec::with_capacity(10000);
    for i in 0..10000 {
        vec.push(i);
    }
    let array = Array::from_slice(vec.as_slice());
    assert_eq!(array.capacity(), 10000);
    for (idx, i) in array.iter().enumerate() {
        assert_eq!(*i, idx);
    }
}
