use crate::data_structs::atomic_bitmap::AtomicBitmap;
use crate::data_structs::atomic_bitmap::Mode::Relaxed;
use crate::data_structs::bitmap::Bitmap;

#[test]
fn bitmap_init_test() {
    let bitmap = AtomicBitmap::new(10);
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
    let mut bitmap = AtomicBitmap::new(10);
    bitmap.set(0, false, Relaxed);
    bitmap.set(1, true, Relaxed);
    bitmap.set(2, true, Relaxed);

    assert_eq!(bitmap.get(0, Relaxed), Some(false));
    assert_eq!(bitmap.get(1, Relaxed), Some(true));
    assert_eq!(bitmap.get(2, Relaxed), Some(true));
    for i in 3..10 {
        assert_eq!(bitmap.get(i, Relaxed), Some(false));
    }
}

#[test]
fn bitmap_set_get_test2() {
    let mut bitmap = AtomicBitmap::new(129);
    bitmap.set(0, false, Relaxed);
    bitmap.set(1, true, Relaxed);
    bitmap.set(2, true, Relaxed);
    bitmap.set(128, true, Relaxed);

    assert_eq!(bitmap.get(0, Relaxed), Some(false));
    assert_eq!(bitmap.get(1, Relaxed), Some(true));
    assert_eq!(bitmap.get(2, Relaxed), Some(true));
    assert_eq!(bitmap.get(128, Relaxed), Some(true));
    for i in 3..127 {
        assert_eq!(bitmap.get(i, Relaxed), Some(false));
    }
}

#[test]
#[should_panic]
fn bitmap_over_capacity_test() {
    let mut bitmap = AtomicBitmap::new(1);
    bitmap.set(0, true, Relaxed);
    assert_eq!(bitmap.get(0, Relaxed), Some(true));
    bitmap.set(1, true, Relaxed);
}

#[test]
#[should_panic]
fn bitmap_over_capacity_test2() {
    let mut bitmap = AtomicBitmap::new(1);
    bitmap.set(0, true, Relaxed);
    assert_eq!(bitmap.get(0, Relaxed), Some(true));
    bitmap.set(1, true, Relaxed);
}

#[test]
fn bitmap_get_over_capacity_test() {
    let mut bitmap = AtomicBitmap::new(10);
    bitmap.set(9, true, Relaxed);
    assert_eq!(bitmap.get(9, Relaxed), Some(true));
    assert_eq!(bitmap.get(10, Relaxed), None);
}

#[test]
fn bitmap_set_get_test_unchecked() {
    let mut bitmap = AtomicBitmap::new(10);
    unsafe {
        bitmap.set_unchecked(0, false, Relaxed);
        bitmap.set_unchecked(1, true, Relaxed);
        bitmap.set_unchecked(2, true, Relaxed);

        assert!(!bitmap.get_unchecked(0, Relaxed));
        assert!(bitmap.get_unchecked(1, Relaxed));
        assert!(bitmap.get_unchecked(2, Relaxed));

        for i in 3..10 {
            assert!(!bitmap.get_unchecked(i, Relaxed));
        }
    }
}

#[test]
fn test_bitmap_to_indices() {
    let mut bitmap = AtomicBitmap::new(10);
    bitmap.set(0, true, Relaxed);
    bitmap.set(1, true, Relaxed);
    bitmap.set(2, true, Relaxed);
    bitmap.set(3, false, Relaxed);
    bitmap.set(4, true, Relaxed);
    bitmap.set(5, true, Relaxed);
    bitmap.set(6, false, Relaxed);
    bitmap.set(7, true, Relaxed);
    bitmap.set(8, false, Relaxed);

    let indices = bitmap.to_indices_true(Relaxed);
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
    let mut bitmap = AtomicBitmap::new(10);
    bitmap.set(1, true, Relaxed);
    bitmap.set(2, true, Relaxed);
    bitmap.set(4, true, Relaxed);
    bitmap.set(5, true, Relaxed);
    bitmap.set(7, true, Relaxed);

    let indices = bitmap.to_indices_false(Relaxed);
    assert_eq!(indices.len(), 5);
    assert_eq!(indices[0], 0);
    assert_eq!(indices[1], 3);
    assert_eq!(indices[2], 6);
    assert_eq!(indices[3], 8);
    assert_eq!(indices[4], 9);
}
