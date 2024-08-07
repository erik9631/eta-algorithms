use std::alloc::{Layout, realloc};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
pub mod iterator;

pub struct Array<T> {
    phantom_data: PhantomData<T>, // For compile time borrow checking correctness
    layout: Layout,
    data: *mut T,
    capacity: usize,
}

impl<T> Array<T> {
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    pub fn extend(&mut self, new_capacity: usize){
        let new_layout = Layout::array::<T>(new_capacity).expect("Failed to create layout");
        let new_ptr = unsafe{realloc(self.data as *mut u8, new_layout, new_layout.size())};
        if new_ptr.is_null(){
            panic!("Failed to allocate memory");
        }
        self.data = new_ptr as *mut T;
        self.capacity = new_capacity;
        self.layout = new_layout;
    }
    pub fn extend_by(&mut self, additional_capacity: usize){
        self.extend(self.capacity + additional_capacity);
    }

    pub fn new(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).expect("Failed to create layout");
        let data = unsafe { std::alloc::alloc(layout) as *mut T };
        if data.is_null(){
            panic!("Failed to allocate memory");
        }
        return Array {
            phantom_data: PhantomData,
            layout,
            data,
            capacity,
        };
    }
    pub fn iter(&self) -> iterator::ArrayIterator<T> {
        return iterator::ArrayIterator {
            phantom_data: &self.phantom_data,
            data: self.data,
            end: unsafe { self.data.offset(self.capacity as isize) },
        };
    }

    pub fn iter_mut(&mut self) -> iterator::ArrayIteratorMut<T> {
        return iterator::ArrayIteratorMut {
            phantom_data: &mut self.phantom_data,
            data: self.data,
            end: unsafe { self.data.offset(self.capacity as isize) },
        };
    }
}

impl<T> Drop for Array<T>{
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.data as *mut u8, self.layout);
        }
    }
}

impl<T> Index<usize> for Array<T>{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.capacity {
            panic!("Index out of bounds");
        }

        unsafe {
            return &*self.data.offset(index as isize);
        }
    }
}

impl<T> IndexMut<usize> for Array<T>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.capacity {
            panic!("Index out of bounds");
        }
        unsafe {
            return &mut *self.data.offset(index as isize);
        }
    }
}
