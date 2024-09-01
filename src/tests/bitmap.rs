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
