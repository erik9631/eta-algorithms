use crate::utils::{closest_pow2, rotate_inc};
use std::alloc::{dealloc, realloc, Layout};
use std::ptr;

pub struct Queue<T>
where
    T: Copy + Sized,
{
    capacity: usize,
    len: usize,
    layout: Layout,
    data: *mut T,
    front: usize,
    end: usize,
}

impl<T> Queue<T>
where
    T: Copy + Sized,
{
    pub fn new_pow2_sized(capacity: usize) -> Self {
        let capacity = closest_pow2(capacity);
        let layout = Layout::array::<T>(capacity).expect("Failed to create layout");
        let data = unsafe { std::alloc::alloc(layout) as *mut T };
        if data.is_null() {
            panic!("Failed to allocate memory");
        }

        Queue {
            capacity,
            layout,
            data,
            len: 0,
            front: 0,
            end: 0,
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn extend_pow2_sized(&mut self, capacity_pow: usize) {
        let new_capacity = closest_pow2(capacity_pow);
        if new_capacity <= self.capacity {
            panic!("New capacity is less than or equal to current capacity");
        }

        let new_layout = Layout::array::<T>(new_capacity).expect("Failed to create layout");
        // Data can only wrap if data actually exists. We need to do len check.
        if self.front >= self.end && self.len > 0 {
            let new_data = unsafe { std::alloc::alloc(new_layout) as *mut T };
            if new_data.is_null() {
                panic!("Failed to allocate memory");
            }
            let from_front_to_array_end_len = self.capacity() - self.front;
            let from_start_to_end_len = self.front;
            unsafe {
                ptr::copy_nonoverlapping(
                    self.data.add(self.front),
                    new_data,
                    from_front_to_array_end_len,
                )
            }; // After Front
            unsafe {
                ptr::copy_nonoverlapping(
                    self.data,
                    new_data.add(from_front_to_array_end_len),
                    from_start_to_end_len,
                )
            }; // Before Front

            unsafe { dealloc(self.data as *mut u8, self.layout) };
            self.capacity = new_capacity;
            self.data = new_data;
            self.front = 0;
            self.end = from_front_to_array_end_len + from_start_to_end_len;
            return;
        }

        unsafe {
            self.data = realloc(self.data as *mut u8, new_layout, new_layout.size()) as *mut T;
            if self.data.is_null() {
                panic!("Failed to reallocate memory");
            }
            self.capacity = new_capacity;
        }
    }
    #[inline(always)]
    pub fn extend_pow2_sized_by(&mut self, capacity_pow: usize) {
        if self.capacity < self.capacity + capacity_pow {
            return;
        }
        let new_capacity = closest_pow2(self.capacity + capacity_pow);
        self.extend_pow2_sized(new_capacity)
    }
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            panic!("Queue is full");
        }
        unsafe {
            self.data.add(self.end).write(value);
            self.len += 1;
            self.end = rotate_inc(self.end, self.capacity - 1);
        }
    }
    #[inline(always)]
    pub fn front(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        }
        unsafe { Some(self.data.add(self.front).as_ref().unwrap()) }
    }
    #[inline(always)]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.len == 0 {
            return None;
        }
        unsafe { Some(self.data.add(self.front).as_mut().unwrap()) }
    }
    #[inline(always)]
    pub fn dequeue(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        unsafe {
            let result = Some(self.data.add(self.front).read());
            self.len -= 1;
            self.front = rotate_inc(self.front, self.capacity - 1);
            result
        }
    }
}

impl<T> Drop for Queue<T>
where
    T: Copy + Sized,
{
    fn drop(&mut self) {
        unsafe {
            dealloc(self.data as *mut u8, self.layout);
        }
    }
}
