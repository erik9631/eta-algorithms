use std::alloc::Layout;
use std::ptr;

use crate::data_structs::bitmap::consts::{BIT_SIZE, DIV_SHIFT, MASK};
use crate::data_structs::bitmap::handle::Handle;

pub mod atomic_bitmap;
pub mod handle;

#[cfg(target_pointer_width = "64")]
pub(crate) mod consts {
    pub(crate) const DIV_SHIFT: usize = 6;
    pub(crate) const MASK: usize = 63;
    pub(crate) const BIT_SIZE: usize = 64;
}

#[cfg(target_pointer_width = "32")]
pub(self) mod consts {
    pub(crate) const DIV_SHIFT: usize = 5;
    pub(crate) const MASK: usize = 31;
    pub(crate) const BIT_SIZE: usize = 32;
}

#[cfg(target_pointer_width = "16")]
pub(self) mod consts {
    pub(crate) const DIV_SHIFT: usize = 4;
    pub(crate) const MASK: usize = 15;
    pub(crate) const BIT_SIZE: usize = 16;
}

pub struct Bitmap {
    data: *mut usize,
    bit_capacity: usize,
    capacity: usize,
    layout: Layout,
}

impl Bitmap {
    pub fn new(bit_count: usize) -> Self {
        let size = (bit_count >> DIV_SHIFT) + 1;
        let layout = Layout::array::<usize>(size).expect("Failed to create layout");
        let data = unsafe { std::alloc::alloc(layout) as *mut usize };
        unsafe { ptr::write_bytes(data, 0, size) };
        Bitmap {
            data,
            capacity: size,
            bit_capacity: bit_count,
            layout,
        }
    }

    pub fn first_zero(&self, bit_index: usize) -> usize {
        if bit_index >= self.bit_capacity {
            panic!("Bit index out of bounds");
        }
        unsafe { self.first_zero_unchecked(bit_index) }
    }

    pub fn first_one(&self, bit_index: usize) -> usize {
        if bit_index >= self.bit_capacity {
            panic!("Bit index out of bounds");
        }
        unsafe { self.first_one_unchecked(bit_index) }
    }

    pub unsafe fn first_zero_unchecked(&self, bit_index: usize) -> usize {
        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (MASK);
        let mut current_shift = bit_offset;
        let end = self.data.add(self.capacity);
        let mut data_ptr = self.data.add(offset);
        let mut data = *data_ptr >> bit_offset;
        let mut counter = 0;

        while data & 1 == 1 {
            if current_shift == BIT_SIZE - 1 && data_ptr != end {
                data_ptr = data_ptr.add(1);
                data = *data_ptr;
                current_shift = 0;
                counter += 1;
            }
            data = data >> 1;
            counter += 1;
            current_shift += 1;
        }
        counter + bit_index
    }
    pub unsafe fn first_one_unchecked(&self, bit_index: usize) -> usize {
        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (MASK);
        let mut current_shift = bit_offset;
        let end = self.data.add(self.capacity);
        let mut data_ptr = self.data.add(offset);
        let mut data = *data_ptr >> bit_offset;
        let mut counter = 0;

        while data & 1 == 0 {
            if current_shift == BIT_SIZE - 1 && data_ptr != end {
                data_ptr = data_ptr.add(1);
                data = *data_ptr;
                current_shift = 0;
                counter += 1;
            }
            data = data >> 1;
            counter += 1;
            current_shift += 1;
        }

        counter + bit_index
    }
    pub fn check_batch(&self, handles: &[Handle]) -> bool {
        for handle in handles {
            let val = unsafe { *self.data.add(handle.chunk as usize) };
            if (val & handle.bit_mask) != handle.bit_mask {
                return false;
            }
        }
        true
    }

    pub fn to_indices_true(&self) -> Vec<usize> {
        let mut indices = Vec::new();
        for i in 0..self.bit_capacity {
            if unsafe { self.get_unchecked(i) } {
                indices.push(i);
            }
        }
        indices
    }
    pub fn to_indices_false(&self) -> Vec<usize> {
        let mut indices = Vec::new();
        for i in 0..self.bit_capacity {
            if unsafe { self.get_unchecked(i) == false } {
                indices.push(i);
            }
        }
        indices
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
    pub fn set(&mut self, bit_index: usize, value: bool) {
        if bit_index >= self.bit_capacity {
            panic!("Bit index out of bounds");
        }

        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & MASK;
        unsafe {
            let ptr = self.data.add(offset);
            *ptr = (*ptr & !(1 << bit_offset)) | ((value as usize) << bit_offset);
        }
    }
    #[inline(always)]
    pub fn get(&self, bit_index: usize) -> Option<bool> {
        if bit_index >= self.bit_capacity {
            return None;
        }

        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (MASK);
        unsafe {
            let ptr = self.data.add(offset);
            Some((*ptr & (1 << bit_offset)) != 0)
        }
    }
    #[inline(always)]
    pub unsafe fn set_unchecked(&mut self, bit_index: usize, value: bool) {
        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (MASK);
        unsafe {
            let ptr = self.data.add(offset);
            *ptr = (*ptr & !(1 << bit_offset)) | ((value as usize) << bit_offset);
        }
    }
    #[inline(always)]
    pub unsafe fn get_unchecked(&self, bit_index: usize) -> bool {
        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (MASK);
        unsafe {
            let ptr = self.data.add(offset);
            (*ptr & (1 << bit_offset)) != 0
        }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.data as *mut u8, self.layout);
        }
    }
}
