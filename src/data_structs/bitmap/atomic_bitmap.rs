use std::alloc::Layout;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::data_structs::bitmap::consts::{DIV_SHIFT, MASK};
use crate::data_structs::bitmap::handle::Handle;

#[derive(Clone, Copy)]
pub enum Mode {
    Relaxed,
    Strict,
}

pub struct AtomicBitmap {
    data: *mut AtomicUsize,
    bit_capacity: usize,
    capacity: usize,
    layout: Layout,
}

impl AtomicBitmap {
    pub fn new(bit_count: usize) -> Self {
        let size = (bit_count >> DIV_SHIFT) + 1;
        let layout = Layout::array::<AtomicUsize>(size).expect("Failed to create layout");
        let data = unsafe { std::alloc::alloc(layout) as *mut AtomicUsize };
        for i in 0..size {
            unsafe {
                (*data.add(i)).store(0, Ordering::Relaxed);
            }
        }
        AtomicBitmap {
            data,
            capacity: size,
            bit_capacity: bit_count,
            layout,
        }
    }

    pub fn to_indices_true(&self, mode: Mode) -> Vec<usize> {
        let mut indices = Vec::new();
        for i in 0..self.bit_capacity {
            if unsafe { self.get_unchecked(i, mode) } {
                indices.push(i);
            }
        }
        indices
    }
    pub fn to_indices_false(&self, mode: Mode) -> Vec<usize> {
        let mut indices = Vec::new();
        for i in 0..self.bit_capacity {
            if unsafe { self.get_unchecked(i, mode) == false } {
                indices.push(i);
            }
        }
        indices
    }

    pub fn check_batch(&self, handles: &[Handle], mode: Mode) -> bool {
        for handle in handles {
            let val = unsafe {
                match mode {
                    Mode::Relaxed => (*self.data.add(handle.chunk as usize)).load(Ordering::Relaxed),
                    Mode::Strict => (*self.data.add(handle.chunk as usize)).load(Ordering::Acquire),
                }
            };
            if (val & handle.bit_mask) != handle.bit_mask {
                return false;
            }
        }
        true
    }

    #[inline(always)]
    pub fn bit_capacity(&self) -> usize {
        self.bit_capacity
    }
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    #[inline(always)]
    pub fn set(&self, bit_index: usize, value: bool, mode: Mode) {
        if bit_index >= self.bit_capacity {
            panic!("Bit index out of bounds");
        }

        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (MASK);
        unsafe {
            let ptr = self.data.add(offset);
            match mode {
                Mode::Relaxed => (*ptr).store(
                    ((*ptr).load(Ordering::Relaxed) & !(1 << bit_offset)) | ((value as usize) << bit_offset),
                    Ordering::Relaxed,
                ),
                Mode::Strict => (*ptr).store(
                    ((*ptr).load(Ordering::Acquire) & !(1 << bit_offset)) | ((value as usize) << bit_offset),
                    Ordering::Release,
                ),
            }
        }
    }
    #[inline(always)]
    pub fn get(&self, bit_index: usize, mode: Mode) -> Option<bool> {
        if bit_index >= self.bit_capacity {
            return None;
        }

        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (MASK);
        unsafe {
            let ptr = self.data.add(offset);
            match mode {
                Mode::Relaxed => Some(((*ptr).load(Ordering::Relaxed) & (1 << bit_offset)) != 0),
                Mode::Strict => Some(((*ptr).load(Ordering::Acquire) & (1 << bit_offset)) != 0),
            }
        }
    }
    #[inline(always)]
    pub unsafe fn set_unchecked(&self, bit_index: usize, value: bool, mode: Mode) {
        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (MASK);
        unsafe {
            let ptr = self.data.add(offset);
            match mode {
                Mode::Relaxed => (*ptr).store(
                    ((*ptr).load(Ordering::Relaxed) & !(1 << bit_offset)) | ((value as usize) << bit_offset),
                    Ordering::Relaxed,
                ),
                Mode::Strict => (*ptr).store(
                    ((*ptr).load(Ordering::Acquire) & !(1 << bit_offset)) | ((value as usize) << bit_offset),
                    Ordering::Release,
                ),
            }
        }
    }
    #[inline(always)]
    pub unsafe fn get_unchecked(&self, bit_index: usize, mode: Mode) -> bool {
        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (MASK);
        unsafe {
            let ptr = self.data.add(offset);
            match mode {
                Mode::Relaxed => (*ptr).load(Ordering::Relaxed) & (1 << bit_offset) != 0,
                Mode::Strict => (*ptr).load(Ordering::Acquire) & (1 << bit_offset) != 0,
            }
        }
    }
}

impl Drop for AtomicBitmap {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.data as *mut u8, self.layout);
        }
    }
}

unsafe impl Send for AtomicBitmap {}

unsafe impl Sync for AtomicBitmap {}
