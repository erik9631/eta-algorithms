use crate::data_structs::bitmap::Bitmap;

#[test]
fn bitmap_init_test(){
    let mut bitmap = Bitmap::new(10);
    assert_eq!(bitmap.bit_capacity(), 10);
    assert_eq!(bitmap.capacity(), 1);

    let mut bitmap = Bitmap::new(100);
    assert_eq!(bitmap.bit_capacity(), 100);
    assert_eq!(bitmap.capacity(), 2);

    let mut bitmap = Bitmap::new(1000);
    assert_eq!(bitmap.bit_capacity(), 1000);
    assert_eq!(bitmap.capacity(), 16);
}

#[test]
fn bitmap_set_get_test(){
    let mut bitmap = Bitmap::new(10);
    bitmap.set(0, false);
    bitmap.set(1, true);
    bitmap.set(2, true);

    assert_eq!(bitmap.get(0), Some(false));
    assert_eq!(bitmap.get(1), Some(true));
    assert_eq!(bitmap.get(2), Some(true));
    for i in 3..10{
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
fn bitmap_set_get_test_unchecked(){
    let mut bitmap = Bitmap::new(10);
    unsafe{
        bitmap.set_unchecked(0, false);
        bitmap.set_unchecked(1, true);
        bitmap.set_unchecked(2, true);

        assert_eq!(bitmap.get_unchecked(0), false);
        assert_eq!(bitmap.get_unchecked(1), true);
        assert_eq!(bitmap.get_unchecked(2), true);

        for i in 3..10{
            assert_eq!(bitmap.get_unchecked(i), false);
        }
    }
}