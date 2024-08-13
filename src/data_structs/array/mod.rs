use std::alloc::{Layout, realloc};
use std::cmp::min;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::ptr;
use std::ptr::copy_nonoverlapping;

pub mod iterator;

pub struct Array<T>
where
    T: Copy + Sized,
{
    phantom_data: PhantomData<T>, // For compile time borrow checking correctness
    layout: Layout,
    data: *mut T,
    capacity: usize,
}

impl <T> Clone for Array<T>
where
        T: Copy + Sized
{
    fn clone(&self) -> Self {
        let array = Self::new(self.capacity);
        unsafe{copy_nonoverlapping(self.data, array.data, self.capacity)}
        array
    }
}

impl<T> Array<T>
where
    T: Copy + Sized
{
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
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
    #[inline(always)]
    pub fn extend_by(&mut self, additional_capacity: usize){
        self.extend(self.capacity + additional_capacity);
    }

    pub fn as_ptr(&self) -> *const T {
        self.data
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }

    pub fn new(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).expect("Failed to create layout");
        let data = unsafe { std::alloc::alloc(layout) as *mut T };
        if data.is_null(){
            panic!("Failed to allocate memory");
        }
        Array {
            phantom_data: PhantomData,
            layout,
            data,
            capacity,
        }
    }

    #[inline(always)]
    pub fn new_default_bytes(capacity: usize, default: u8) -> Self {
        let arr = Self::new(capacity);
        unsafe{ptr::write_bytes(arr.data, default, capacity)};
        arr
    }
    #[inline(always)]
    pub fn new_with_default(capacity: usize, default: T) -> Self
    where
        T: Copy,
    {
        let mut arr = Self::new(capacity);
        arr.fill(default);
        arr
    }

    pub fn fill(&mut self, value: T)
    where
        T: Copy,
    {
        for i in self.iter_mut() {
            *i = value;
        }
    }
    #[inline(always)]
    pub fn iter(&self) -> iterator::ArrayIterator<T> {
        iterator::ArrayIterator {
            phantom_data: &self.phantom_data,
            data: self.data,
            end: unsafe { self.data.add(self.capacity) },
        }
    }
    #[inline(always)]
    pub fn iter_mut(&mut self) -> iterator::ArrayIteratorMut<T> {
        iterator::ArrayIteratorMut {
            phantom_data: &mut self.phantom_data,
            data: self.data,
            end: unsafe { self.data.add(self.capacity) },
        }
    }

    #[inline(always)]
    pub fn iter_range(&self, start: usize, end: usize) -> iterator::ArrayIterator<T> {
        iterator::ArrayIterator {
            phantom_data: &self.phantom_data,
            data: unsafe{self.data.add(start)},
            end: unsafe{min(self.data.add(start + end), self.data.add(self.capacity))},
        }
    }

    #[inline(always)]
    pub fn iter_range_mut(&mut self, start: usize, end: usize) -> iterator::ArrayIteratorMut<T> {
        iterator::ArrayIteratorMut {
            phantom_data: &mut self.phantom_data,
            data: unsafe{self.data.add(start)},
            end: unsafe{min(self.data.add(start + end), self.data.add(self.capacity))},
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe{std::slice::from_raw_parts(self.data, self.capacity)}
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe{std::slice::from_raw_parts_mut(self.data, self.capacity)}
    }

    pub fn split_at(&mut self, index: usize) -> (&[T], &[T]) {
        if index >= self.capacity {
            panic!("Index out of bounds");
        }
        let new_data = unsafe{self.data.add(index)};
        let left = unsafe{std::slice::from_raw_parts(self.data, index)};
        let right = unsafe{std::slice::from_raw_parts(new_data, self.capacity - index)};
        (left, right)
    }

    pub fn split_at_mut(&mut self, index: usize) -> (&mut [T], &mut [T]) {
        if index >= self.capacity {
            panic!("Index out of bounds");
        }
        let new_data = unsafe{self.data.add(index)};
        let left = unsafe{std::slice::from_raw_parts_mut(self.data, index)};
        let right = unsafe{std::slice::from_raw_parts_mut(new_data, self.capacity - index)};
        (left, right)
    }

    pub fn split_into_parts(&self, parts: usize) -> Array<&[T]>{
        if parts > self.capacity {
            panic!("Parts must be less than or equal to the capacity of the array");
        }

        if parts == 0 {
            panic!("Parts cannot be 0");
        }

        let chunk_size = self.capacity / parts;
        let remainder = self.capacity % parts;

        let mut arr = Array::<&[T]>::new(parts);
        let mut ptr = self.data as *const T;
        for i in 0..parts -1 {
            arr[i] = unsafe{std::slice::from_raw_parts(ptr, chunk_size)};
            ptr = unsafe{ptr.add(chunk_size)};
        }

        arr[parts - 1] = unsafe{std::slice::from_raw_parts(ptr, chunk_size + remainder)};

        arr
    }
    pub fn split_into_parts_mut(&mut self, parts: usize) -> Vec<&mut[T]> {
        if parts >= self.capacity {
            panic!("Parts must be less than or equal to the capacity of the array");
        }

        if parts == 0 {
            panic!("Parts cannot be 0");
        }

        let chunk_size = self.capacity / parts;
        let remainder = self.capacity % parts;

        let mut arr = Vec::<&mut [T]>::with_capacity(parts);
        let mut ptr = self.data;
        for i in 0..parts - 1 {
            arr.push(unsafe{std::slice::from_raw_parts_mut(ptr, chunk_size)});
            ptr = unsafe{ptr.add(chunk_size)};
        }

        arr.push(unsafe{std::slice::from_raw_parts_mut(ptr, chunk_size + remainder)});
        arr
    }
    pub unsafe fn index_unchecked(&self, index: usize) -> &T {
         self.data.add(index).as_ref().unwrap()
    }

    pub unsafe fn index_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.data.add(index).as_mut().unwrap()
    }

}


impl<T> Drop for Array<T>
where
    T: Copy + Sized,
{

    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.data as *mut u8, self.layout);
        }
    }
}

impl<T> Index<usize> for Array<T>
where
    T: Copy + Sized,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.capacity {
            panic!("Index out of bounds");
        }

        unsafe {
            &*self.data.add(index)
        }
    }
}

impl<T> IndexMut<usize> for Array<T>
where
    T: Copy + Sized,
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.capacity {
            panic!("Index out of bounds");
        }
        unsafe {
            &mut *self.data.add(index)
        }
    }
}
