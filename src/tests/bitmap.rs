use crate::data_structs::bitmap::handle::Handle;
use crate::data_structs::bitmap::Bitmap;

#[test]
fn bitmap_init_test() {
    let bitmap = Bitmap::new(10);
    assert_eq!(bitmap.bit_capacity(), 10);
    assert_eq!(bitmap.capacity(), 1);

    let bitmap = Bitmap::new(100);
    assert_eq!(bitmap.bit_capacity(), 100);
    assert_eq!(bitmap.capacity(), 2);

    let bitmap = Bitmap::new(1000);
    assert_eq!(bitmap.bit_capacity(), 1000);
    assert_eq!(bitmap.capacity(), 16);
}

#[test]
fn bitmap_set_get_test() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, false);
    bitmap.set(1, true);
    bitmap.set(2, true);

    assert_eq!(bitmap.get(0), Some(false));
    assert_eq!(bitmap.get(1), Some(true));
    assert_eq!(bitmap.get(2), Some(true));
    for i in 3..10 {
        assert_eq!(bitmap.get(i), Some(false));
    }
}

#[test]
fn bitmap_set_get_test2() {
    let mut bitmap = Bitmap::new(129);
    bitmap.set(0, false);
    bitmap.set(1, true);
    bitmap.set(2, true);
    bitmap.set(128, true);

    assert_eq!(bitmap.get(0), Some(false));
    assert_eq!(bitmap.get(1), Some(true));
    assert_eq!(bitmap.get(2), Some(true));
    assert_eq!(bitmap.get(128), Some(true));
    for i in 3..127 {
        assert_eq!(bitmap.get(i), Some(false));
    }
}

#[test]
#[should_panic]
fn bitmap_over_capacity_test() {
    let mut bitmap = Bitmap::new(1);
    bitmap.set(0, true);
    assert_eq!(bitmap.get(0), Some(true));
    bitmap.set(1, true);
}

#[test]
#[should_panic]
fn bitmap_over_capacity_test2() {
    let mut bitmap = Bitmap::new(1);
    bitmap.set(0, true);
    assert_eq!(bitmap.get(0), Some(true));
    bitmap.set(1, true);
}

#[test]
fn bitmap_get_over_capacity_test() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(9, true);
    assert_eq!(bitmap.get(9), Some(true));
    assert_eq!(bitmap.get(10), None);
}

#[test]
fn bitmap_set_get_test_unchecked() {
    let mut bitmap = Bitmap::new(10);
    unsafe {
        bitmap.set_unchecked(0, false);
        bitmap.set_unchecked(1, true);
        bitmap.set_unchecked(2, true);

        assert!(!bitmap.get_unchecked(0));
        assert!(bitmap.get_unchecked(1));
        assert!(bitmap.get_unchecked(2));

        for i in 3..10 {
            assert!(!bitmap.get_unchecked(i));
        }
    }
}

#[test]
fn test_bitmap_to_indices() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, true);
    bitmap.set(1, true);
    bitmap.set(2, true);
    bitmap.set(3, false);
    bitmap.set(4, true);
    bitmap.set(5, true);
    bitmap.set(6, false);
    bitmap.set(7, true);
    bitmap.set(8, false);

    let indices = bitmap.to_indices_true();
    assert_eq!(indices.len(), 6);
    assert_eq!(indices[0], 0);
    assert_eq!(indices[1], 1);
    assert_eq!(indices[2], 2);
    assert_eq!(indices[3], 4);
    assert_eq!(indices[4], 5);
    assert_eq!(indices[5], 7);
}

#[test]
fn bitmap_to_indices_false_test() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(1, true);
    bitmap.set(2, true);
    bitmap.set(4, true);
    bitmap.set(5, true);
    bitmap.set(7, true);

    let indices = bitmap.to_indices_false();
    assert_eq!(indices.len(), 5);
    assert_eq!(indices[0], 0);
    assert_eq!(indices[1], 3);
    assert_eq!(indices[2], 6);
    assert_eq!(indices[3], 8);
    assert_eq!(indices[4], 9);
}

#[test]
fn bitmap_batch_test_single() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, true);
    bitmap.set(1, true);
    bitmap.set(2, true);
    bitmap.set(3, false);
    bitmap.set(4, true);
    bitmap.set(5, true);
    bitmap.set(6, false);
    bitmap.set(7, true);
    bitmap.set(8, false);
    let handles = Handle::new_batch(&[0, 1, 2, 4, 5, 7]);
    assert_eq!(handles.capacity(), 1);
    assert!(bitmap.check_batch(handles.as_slice()));
}

#[test]
fn bitmap_batch_test_single_false() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, true);
    bitmap.set(1, true);
    bitmap.set(2, true);
    bitmap.set(3, false);
    bitmap.set(4, true);
    bitmap.set(5, true);
    bitmap.set(6, false);
    bitmap.set(7, true);
    bitmap.set(8, false);
    let handles = Handle::new_batch(&[0, 1, 2, 4, 5, 7, 8]);
    assert_eq!(bitmap.check_batch(handles.as_slice()), false);
}

#[test]
fn bitmap_test_batch() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, i % 2 == 0);
    }
    let handles = Handle::new_batch(&[0, 4, 1022]);
    assert_eq!(bitmap.check_batch(handles.as_slice()), true);
}

#[test]
fn bitmap_test_batch_fail() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, i % 2 == 0);
    }
    let handles = Handle::new_batch(&[0, 4, 1023]);
    assert_eq!(bitmap.check_batch(handles.as_slice()), false);
}

#[test]
fn bitmap_test_batch_overlapping() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, i % 2 == 0);
    }
    let handles = Handle::new_batch(&[0, 4, 6, 8, 10, 12, 14, 1022]);
    assert_eq!(bitmap.check_batch(handles.as_slice()), true);
}

#[test]
fn bitmap_test_first_zero_1() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, true);
    bitmap.set(1, true);
    bitmap.set(2, true);
    bitmap.set(3, false);
    bitmap.set(4, true);
    bitmap.set(5, true);
    bitmap.set(6, false);
    bitmap.set(7, true);
    bitmap.set(8, false);

    assert_eq!(bitmap.first_zero(0), Some(3));
}
#[test]
fn bitmap_test_first_zero_2() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, false);
    bitmap.set(1, false);
    bitmap.set(2, false);
    bitmap.set(3, false);
    bitmap.set(4, true);
    bitmap.set(5, true);
    bitmap.set(6, false);
    bitmap.set(7, true);
    bitmap.set(8, false);

    assert_eq!(bitmap.first_zero(0), Some(0));
}

#[test]
fn bitmap_test_first_zero_3() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, true);
    bitmap.set(1, true);
    bitmap.set(2, true);
    bitmap.set(3, true);
    bitmap.set(4, true);
    bitmap.set(5, true);
    bitmap.set(6, true);
    bitmap.set(7, true);
    bitmap.set(8, false);

    assert_eq!(bitmap.first_zero(0), Some(8));
}
#[test]
fn bitmap_test_first_zero_4() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, true);
    }

    bitmap.set(70, false);

    assert_eq!(bitmap.first_zero(35), Some(70));
}

#[test]
fn bitmap_test_first_zero_5() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, true);
    }

    bitmap.set(1000, false);

    assert_eq!(bitmap.first_zero(35), Some(1000));
}

#[test]
fn bitmap_test_first_zero_6() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, true);
    }

    bitmap.set(1000, false);
    bitmap.set(1001, false);
    bitmap.set(1002, false);

    assert_eq!(bitmap.first_zero(35), Some(1000));
}

#[test]
fn bitmap_test_first_zero_7() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, true);
    }

    bitmap.set(1000, false);
    bitmap.set(1001, false);
    bitmap.set(1002, false);

    assert_eq!(bitmap.first_zero(1000), Some(1000));
}

#[test]
fn bitmap_test_first_one_1() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, false);
    bitmap.set(1, false);
    bitmap.set(2, false);
    bitmap.set(3, true);
    bitmap.set(4, false);
    bitmap.set(5, false);
    bitmap.set(6, true);
    bitmap.set(7, false);
    bitmap.set(8, true);

    assert_eq!(bitmap.first_one(0), Some(3));
}
#[test]
fn bitmap_test_first_one_2() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, true);
    bitmap.set(1, false);
    bitmap.set(2, false);
    bitmap.set(3, false);
    bitmap.set(4, true);
    bitmap.set(5, true);
    bitmap.set(6, false);
    bitmap.set(7, true);
    bitmap.set(8, false);

    assert_eq!(bitmap.first_one(0), Some(0));
}

#[test]
fn bitmap_test_first_one_3() {
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, false);
    bitmap.set(1, false);
    bitmap.set(2, false);
    bitmap.set(3, false);
    bitmap.set(4, false);
    bitmap.set(5, false);
    bitmap.set(6, false);
    bitmap.set(7, false);
    bitmap.set(8, true);

    assert_eq!(bitmap.first_one(0), Some(8));
}
#[test]
fn bitmap_test_first_one_4() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, false);
    }

    bitmap.set(70, true);

    assert_eq!(bitmap.first_one(35), Some(70));
}

#[test]
fn bitmap_test_first_one_5() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, false);
    }

    bitmap.set(1000, true);

    assert_eq!(bitmap.first_one(35), Some(1000));
}

#[test]
fn bitmap_test_first_one_6() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, false);
    }

    bitmap.set(1000, true);
    bitmap.set(1001, true);
    bitmap.set(1002, true);

    assert_eq!(bitmap.first_one(35), Some(1000));
}

#[test]
fn bitmap_test_first_one_7() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, false);
    }

    bitmap.set(1000, true);
    bitmap.set(1001, true);
    bitmap.set(1002, true);

    assert_eq!(bitmap.first_one(1000), Some(1000));
}

#[test]
fn bitmap_test_first_one_8() {
    let mut bitmap = Bitmap::new(1024);
    for i in 0..1024 {
        bitmap.set(i, false);
    }

    bitmap.set(1000, true);
    bitmap.set(1001, true);
    bitmap.set(1002, true);

    assert_eq!(bitmap.first_one(0), Some(1000));
}

#[test]
fn bitmap_test_first_zero_end() {
    let mut bitmap = Bitmap::new(64);
    for i in 0..64 {
        bitmap.set(i, true);
    }

    bitmap.set(63, false);

    assert_eq!(bitmap.first_zero(63), Some(63));
}

#[test]
fn bitmap_test_first_one_end() {
    let mut bitmap = Bitmap::new(64);
    for i in 0..64 {
        bitmap.set(i, false);
    }

    bitmap.set(63, true);

    assert_eq!(bitmap.first_one(63), Some(63));
}

#[test]
fn bitmap_test_no_one_test() {
    let mut bitmap = Bitmap::new(64);
    for i in 0..64 {
        bitmap.set(i, false);
    }
    assert_eq!(bitmap.first_one(0), None);
}
#[test]
fn bitmap_test_no_zero_test() {
    let mut bitmap = Bitmap::new(64);
    for i in 0..64 {
        bitmap.set(i, true);
    }
    assert_eq!(bitmap.first_zero(0), None);
}

#[test]
fn bitmap_test_upper_bound_zero_test_1() {
    let mut bitmap = Bitmap::new(64);
    for i in 0..64 {
        bitmap.set(i, true);
    }
    bitmap.set(5, false);

    assert_eq!(bitmap.first_zero_bounds(4, 7), Some(5));
}

#[test]
fn bitmap_test_upper_bound_zero_test_2() {
    let mut bitmap = Bitmap::new(100);
    for i in 0..100 {
        bitmap.set(i, true);
    }
    bitmap.set(65, false);
    bitmap.set(66, false);
    bitmap.set(67, false);

    assert_eq!(bitmap.first_zero_bounds(60, 66), Some(65));
}

#[test]
fn bitmap_test_upper_bound_zero_test_3() {
    let mut bitmap = Bitmap::new(1000);
    for i in 0..1000 {
        bitmap.set(i, true);
    }
    bitmap.set(991, false);
    bitmap.set(999, false);

    assert_eq!(bitmap.first_zero_bounds(992, 1000), Some(999));
}

#[test]
fn bitmap_test_upper_bound_one_test_1() {
    let mut bitmap = Bitmap::new(64);
    for i in 0..64 {
        bitmap.set(i, false);
    }
    bitmap.set(5, true);

    assert_eq!(bitmap.first_one_bounds(4, 7), Some(5));
}

#[test]
fn bitmap_test_upper_bound_one_test_2() {
    let mut bitmap = Bitmap::new(100);
    for i in 0..100 {
        bitmap.set(i, false);
    }
    bitmap.set(65, true);
    bitmap.set(66, true);
    bitmap.set(67, true);

    assert_eq!(bitmap.first_one_bounds(60, 66), Some(65));
}

#[test]
fn bitmap_test_upper_bound_one_test_3() {
    let mut bitmap = Bitmap::new(1000);
    for i in 0..1000 {
        bitmap.set(i, false);
    }
    bitmap.set(991, true);
    bitmap.set(999, true);

    assert_eq!(bitmap.first_one_bounds(992, 1000), Some(999));
}

#[test]
fn bitmap_test_count_one_1() {
    let mut bitmap = Bitmap::new(100);
    for i in 0..100 {
        bitmap.set(i, true);
    }
    assert_eq!(bitmap.count_ones(0, 100), 100);
    assert_eq!(bitmap.count_ones(25, 100), 75);
    assert_eq!(bitmap.count_ones(50, 100), 50);
    assert_eq!(bitmap.count_ones(25, 100), 75);
    assert_eq!(bitmap.count_ones(99, 100), 1);
}

#[test]
fn bitmap_test_count_one_2() {
    let mut bitmap = Bitmap::new(100);
    for i in 0..100 {
        bitmap.set(i, i % 2 == 0);
    }
    assert_eq!(bitmap.count_ones(0, 100), 50);
    assert_eq!(bitmap.count_ones(25, 100), 37);
}

#[test]
fn bitmap_test_count_one_3() {
    let mut bitmap = Bitmap::new(65);
    for i in 0..65 {
        bitmap.set(i, false);
    }
    bitmap.set(64, true);
    assert_eq!(bitmap.count_ones(64, 65), 1);
}

#[test]
fn bitmap_test_count_zero_1() {
    let mut bitmap = Bitmap::new(100);
    for i in 0..100 {
        bitmap.set(i, false);
    }
    assert_eq!(bitmap.count_zeros(0, 100), 100); // Expected behavior as unbounded search looks till the end of the memory
    assert_eq!(bitmap.count_zeros(25, 100), 75);
    assert_eq!(bitmap.count_zeros(50, 100), 50);
    assert_eq!(bitmap.count_zeros(99, 100), 1);
}

#[test]
fn bitmap_test_count_zero_2() {
    let mut bitmap = Bitmap::new(100);
    for i in 0..100 {
        bitmap.set(i, i % 2 == 1);
    }
    assert_eq!(bitmap.count_zeros(0, 100), 50); // 50 zeros from pattern + 28 additional zeros in allocated memory
    assert_eq!(bitmap.count_zeros(25, 100), 37);
}

#[test]
fn bitmap_test_count_zero_3() {
    let mut bitmap = Bitmap::new(65);
    for i in 0..65 {
        bitmap.set(i, true);
    }
    bitmap.set(64, false);
    assert_eq!(bitmap.count_zeros(64, 65), 1);
}

#[test]
fn bitmap_test_count_zero_eq() {
    let mut bitmap = Bitmap::new(65);
    for i in 0..65 {
        bitmap.set(i, true);
    }
    bitmap.set(64, false);
    assert_eq!(bitmap.count_zeros(64, 64), 0);
}

#[test]
fn bitmap_test_count_one_eq() {
    let mut bitmap = Bitmap::new(65);
    for i in 0..65 {
        bitmap.set(i, true);
    }
    bitmap.set(64, false);
    assert_eq!(bitmap.count_ones(64, 64), 0);
}
#[test]
#[should_panic]
fn bitmap_test_count_one_over() {
    let mut bitmap = Bitmap::new(65);
    for i in 0..65 {
        bitmap.set(i, true);
    }
    bitmap.set(64, false);
    assert_eq!(bitmap.count_ones(64, 66), 0);
}

#[test]
#[should_panic]
fn bitmap_test_count_zero_over() {
    let mut bitmap = Bitmap::new(65);
    for i in 0..65 {
        bitmap.set(i, true);
    }
    bitmap.set(64, false);
    assert_eq!(bitmap.count_zeros(64, 66), 0);
}

// Lower bound out of bounds tests
#[test]
#[should_panic]
fn bitmap_test_count_one_lower_bound_over() {
    let bitmap = Bitmap::new(65);
    bitmap.count_ones(65, 66);
}

#[test]
#[should_panic]
fn bitmap_test_count_zero_lower_bound_over() {
    let bitmap = Bitmap::new(65);
    bitmap.count_zeros(65, 66);
}

// Lower bound greater than upper bound tests
#[test]
#[should_panic]
fn bitmap_test_count_one_invalid_range() {
    let bitmap = Bitmap::new(100);
    bitmap.count_ones(50, 30);
}

#[test]
#[should_panic]
fn bitmap_test_count_zero_invalid_range() {
    let bitmap = Bitmap::new(100);
    bitmap.count_zeros(50, 30);
}

// Single bit ranges (adjacent indices)
#[test]
fn bitmap_test_count_one_single_bit() {
    let mut bitmap = Bitmap::new(100);
    bitmap.set(50, true);
    assert_eq!(bitmap.count_ones(50, 51), 1);
    assert_eq!(bitmap.count_ones(49, 50), 0);
}

#[test]
fn bitmap_test_count_zero_single_bit() {
    let mut bitmap = Bitmap::new(100);
    for i in 0..100 {
        bitmap.set(i, true);
    }
    bitmap.set(50, false);
    assert_eq!(bitmap.count_zeros(50, 51), 1);
    assert_eq!(bitmap.count_zeros(49, 50), 0);
}

// Cross-chunk boundary tests (important for 64-bit boundaries)
#[test]
fn bitmap_test_count_one_cross_chunk() {
    let mut bitmap = Bitmap::new(200);
    // Set bits around 64-bit boundaries
    for i in 60..68 {
        bitmap.set(i, true);
    }
    for i in 124..132 {
        bitmap.set(i, true);
    }

    assert_eq!(bitmap.count_ones(62, 66), 4); // Cross 64-bit boundary
    assert_eq!(bitmap.count_ones(126, 130), 4); // Cross 128-bit boundary
}

#[test]
fn bitmap_test_count_zero_cross_chunk() {
    let mut bitmap = Bitmap::new(200);
    // Set all bits to true, then clear some around boundaries
    for i in 0..200 {
        bitmap.set(i, true);
    }
    for i in 60..68 {
        bitmap.set(i, false);
    }

    assert_eq!(bitmap.count_zeros(62, 66), 4); // Cross 64-bit boundary
    assert_eq!(bitmap.count_zeros(60, 68), 8); // Full range
}

// Same chunk tests (within single usize)
#[test]
fn bitmap_test_count_one_same_chunk() {
    let mut bitmap = Bitmap::new(100);
    // Set some bits within first chunk (0-63)
    bitmap.set(10, true);
    bitmap.set(20, true);
    bitmap.set(30, true);

    assert_eq!(bitmap.count_ones(5, 35), 3);
    assert_eq!(bitmap.count_ones(15, 25), 1);
}

#[test]
fn bitmap_test_count_zero_same_chunk() {
    let mut bitmap = Bitmap::new(100);
    // Set all bits in first chunk to true, then clear some
    for i in 0..64 {
        bitmap.set(i, true);
    }
    bitmap.set(10, false);
    bitmap.set(20, false);
    bitmap.set(30, false);

    assert_eq!(bitmap.count_zeros(5, 35), 3);
    assert_eq!(bitmap.count_zeros(15, 25), 1);
}

// Boundary edge cases
#[test]
fn bitmap_test_count_one_at_capacity() {
    let mut bitmap = Bitmap::new(64);
    for i in 0..64 {
        bitmap.set(i, true);
    }

    assert_eq!(bitmap.count_ones(63, 64), 1); // Last bit
    assert_eq!(bitmap.count_ones(0, 64), 64); // Full capacity
}

#[test]
fn bitmap_test_count_zero_at_capacity() {
    let mut bitmap = Bitmap::new(64);
    bitmap.set(63, false);

    assert_eq!(bitmap.count_zeros(63, 64), 1); // Last bit
    assert_eq!(bitmap.count_zeros(0, 64), 64); // All zeros (initialized state)
}

// Large range spanning many chunks
#[test]
fn bitmap_test_count_one_large_range() {
    let mut bitmap = Bitmap::new(1000);
    // Set every 10th bit to true
    for i in (0..1000).step_by(10) {
        bitmap.set(i, true);
    }

    assert_eq!(bitmap.count_ones(0, 1000), 100);
    assert_eq!(bitmap.count_ones(100, 900), 80); // 80 bits set in this range
}

#[test]
fn bitmap_test_count_zero_large_range() {
    let mut bitmap = Bitmap::new(1000);
    // Set all bits to true, then clear every 10th
    for i in 0..1000 {
        bitmap.set(i, true);
    }
    for i in (0..1000).step_by(10) {
        bitmap.set(i, false);
    }

    assert_eq!(bitmap.count_zeros(0, 1000), 100);
    assert_eq!(bitmap.count_zeros(100, 900), 80);
}

#[test]
fn bitmap_test_count_zero_length_ranges() {
    let bitmap = Bitmap::new(100);

    assert_eq!(bitmap.count_ones(50, 50), 0);
    assert_eq!(bitmap.count_ones(99, 99), 0);

    assert_eq!(bitmap.count_zeros(50, 50), 0);
    assert_eq!(bitmap.count_zeros(99, 99), 0);
}
