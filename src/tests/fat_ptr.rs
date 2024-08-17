use crate::data_structs::array::Array;
use crate::data_structs::fat_ptr::{FatPtr, FatPtrMut};

#[test]
pub fn fat_ptr_test(){
    let mut array = Array::new(10);
    for i in 0..10 {
        array[i] = i;
    }
    let mut fat_ptr = FatPtr::from_slice(array.as_slice());

    for i in 0..10 {
        unsafe{
            assert_eq!(*fat_ptr.ptr, i);
            fat_ptr.ptr = fat_ptr.ptr.offset(1);
        }
    }
}

#[test]
pub fn fat_ptr_mut_test(){
    let mut array = Array::new(10);
    for i in 0..10 {
        array[i] = i;
    }
    let fat_ptr = FatPtrMut::from_mut_slice(array.as_mut_slice());

    for i in 0..10 {
        unsafe{
            *fat_ptr.ptr.add(i) = 100 + i;
        }
    }

    for i in 0..10 {
        unsafe{
            assert_eq!(*fat_ptr.ptr.add(i), 100 + i);
        }
    }
}

#[test]
pub fn fat_ptr_iter_test(){
    let mut array = Array::new(10);
    for i in 0..10 {
        array[i] = i;
    }
    let fat_ptr = FatPtr::from_slice(array.as_slice());

    for (idx, i) in fat_ptr.enumerate() {
        assert_eq!(*i, idx);
    }
}

#[test]
pub fn fat_ptr_mut_iter_test() {
    let mut array = Array::new_default_bytes(10, 0);
    let mut fat_ptr = FatPtrMut::from_mut_slice(array.as_mut_slice());

    for (idx, i) in fat_ptr.enumerate() {
        *i = idx;
    }

    fat_ptr = FatPtrMut::from_mut_slice(array.as_mut_slice());

    for (idx, i) in fat_ptr.enumerate() {
        assert_eq!(*i, idx);
    }
}