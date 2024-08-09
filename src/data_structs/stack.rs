use std::alloc::{Layout, realloc};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::ptr;

pub struct Stack<T> {
    capacity: usize,
    len: usize,
    layout: Layout,
    data: *mut T,
    top: *mut T,
    end: *mut T,

}

impl<T> Stack<T> {
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        return self.capacity;
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        return self.len;
    }
    pub fn new(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).expect("Failed to create layout");
        let data = unsafe { std::alloc::alloc(layout) as *mut T };
        if data.is_null() {
            panic!("Failed to allocate memory");
        }

        return Stack {
            capacity,
            layout,
            data,
            len: 0,
            top: unsafe{data.offset(-1)},
            end: unsafe { data.offset(capacity as isize) },
        };
    }

    pub fn extend(&mut self, new_capacity: usize) {
        let new_layout = Layout::array::<T>(new_capacity).expect("Failed to create layout");
        unsafe {
            self.data = realloc(self.data as *mut u8, new_layout, new_layout.size()) as *mut T;
            if self.data.is_null() {
                panic!("Failed to reallocate memory");
            }
            self.end = self.data.add(new_capacity);
            self.top = self.data.offset((self.len as isize) - 1);
        }
        self.capacity = new_capacity;
        self.layout = new_layout;
    }
    #[inline(always)]
    pub fn extend_by(&mut self, additional_capacity: usize) {
        self.extend(self.capacity + additional_capacity);
    }
    #[inline(always)]
    pub fn top(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        }
        unsafe {
            Some(self.top.as_ref().unwrap())
        }
    }
    #[inline(always)]
    pub fn top_mut(&mut self) -> Option<&mut T> {
        if self.len == 0 {
            return None;
        }
        unsafe {
            Some(self.top.as_mut().unwrap())
        }
    }
    // #[inline(always)]
    pub fn push(&mut self, value: T) {
        let new_top = unsafe{self.top.offset(1)};
        if new_top == self.end {
            panic!("Stack over capacity!");
        }

        self.top = new_top;
        unsafe {self.top.write(value)};
        self.len += 1;
    }
    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            let result = Some(ptr::read(self.top));
            self.top = self.top.offset(-1);
            self.len -= 1;
            result
        }
    }
}

impl<T> Index<isize> for Stack<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, index: isize) -> &Self::Output {
        if index > 0{
            panic!("Index out of bounds");
        }

        if index <= self.len as isize * -1{
            panic!("Index out of bounds");
        }

        let indexed_data = unsafe{self.top.offset(index)};
        unsafe {indexed_data.as_ref().unwrap()}
    }
}

impl<T> IndexMut<isize> for Stack<T>{
    #[inline(always)]
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        if index > 0{
            panic!("Index out of bounds");
        }

        if index < self.len as isize * -1{
            panic!("Index out of bounds");
        }

        let indexed_data = unsafe{self.top.offset(index)};
        unsafe {indexed_data.as_mut().unwrap()}
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.data as *mut u8, self.layout);
        }
    }
}