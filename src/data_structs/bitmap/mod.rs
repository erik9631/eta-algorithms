use std::alloc::Layout;
use std::ptr;

use crate::data_structs::bitmap::consts::{BIT_END_OFFSET, BIT_MASK, DIV_SHIFT};
use crate::data_structs::bitmap::handle::Handle;

pub mod atomic_bitmap;
pub mod handle;

#[cfg(target_pointer_width = "64")]
pub(crate) mod consts {
    pub(crate) const DIV_SHIFT: usize = 6; // Divide by 64 => 2^6
    pub(crate) const BIT_END_OFFSET: usize = 63;
    pub(crate) const BIT_MASK: usize = 0xFFFFFFFFFFFFFFFF;
}

#[cfg(target_pointer_width = "32")]
pub(self) mod consts {
    pub(crate) const DIV_SHIFT: usize = 5;
    pub(crate) const BIT_END_OFFSET: usize = 31;
    pub(crate) const BIT_MASK: usize = 0xFFFFFFFF;
}

#[cfg(target_pointer_width = "16")]
pub(self) mod consts {
    pub(crate) const DIV_SHIFT: usize = 4;
    pub(crate) const BIT_END_OFFSET: usize = 15;
    pub(crate) const BIT_MASK: usize = 0xFFFF;
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

    pub fn count_zeros(&self, lower_bound: usize, upper_bound: usize) -> usize {
        if lower_bound >= self.bit_capacity {
            panic!("Bit index out of bounds");
        }
        if upper_bound >= self.bit_capacity {
            panic!("Upper bound out of bounds");
        }

        if lower_bound == upper_bound {
            return 0;
        }

        unsafe { self.count_zeros_unchecked(lower_bound, upper_bound - 1) }
    }

    pub fn count_ones(&self, lower_bound: usize, upper_bound: usize) -> usize {
        if lower_bound >= self.bit_capacity {
            panic!("Bit index out of bounds");
        }

        if upper_bound >= self.bit_capacity {
            panic!("Upper bound out of bounds");
        }

        if lower_bound == upper_bound {
            return 0;
        }

        unsafe { self.count_ones_unchecked(lower_bound, upper_bound - 1) }
    }

    pub unsafe fn count_zeros_unchecked(&self, lower_bound: usize, upper_bound: usize) -> usize {
        let lower_offset = lower_bound >> DIV_SHIFT;
        let lower_bit_offset = lower_bound & BIT_END_OFFSET;
        let upper_offset = upper_bound >> DIV_SHIFT;
        let upper_bit_offset = upper_bound & BIT_END_OFFSET;
        let mut data_ptr = self.data.add(lower_offset);
        let mut data = if lower_offset != upper_offset {
            *data_ptr | ((1 << lower_bit_offset) - 1)
        } else {
            (*data_ptr | ((1 << lower_bit_offset) - 1)) | !(((1 << upper_bit_offset) - 1) | (1 << upper_bit_offset))
            // + 1 because inclusive and we need to do upper and lower bound masking
        };
        let mut counter = data.count_zeros() as usize;
        let end = self.data.add(upper_offset);
        loop {
            data_ptr = data_ptr.add(1);
            data = *data_ptr;
            if data_ptr >= end {
                break;
            }
            counter += data.count_zeros() as usize;
        }
        // The remainder
        if upper_offset != lower_offset {
            let data_end_ptr = data_ptr;
            // The other OR is there to add 1 because upper_bound is inclusive
            let data_end = *data_end_ptr | !(((1 << upper_bit_offset) - 1) | (1 << upper_bit_offset)); // + 1 because inclusive
            counter += data_end.count_zeros() as usize;
        }

        counter
    }

    pub unsafe fn count_ones_unchecked(&self, lower_bound: usize, upper_bound: usize) -> usize {
        let lower_offset = lower_bound >> DIV_SHIFT;
        let lower_bit_offset = lower_bound & BIT_END_OFFSET;
        let upper_offset = upper_bound >> DIV_SHIFT;
        let upper_bit_offset = upper_bound & BIT_END_OFFSET;
        let mut data_ptr = self.data.add(lower_offset);
        let mut data = if lower_offset != upper_offset {
            *data_ptr & (!((1 << lower_bit_offset) - 1))
        } else {
            (*data_ptr & (!((1 << lower_bit_offset) - 1))) & (((1 << upper_bit_offset) - 1) | (1 << upper_bit_offset))
            // + 1 because inclusive and we need to do upper and lower bound masking
        };
        let mut counter = data.count_ones() as usize;
        let end = self.data.add(upper_offset);
        loop {
            data_ptr = data_ptr.add(1);
            data = *data_ptr;
            if data_ptr >= end {
                break;
            }
            counter += data.count_ones() as usize;
        }
        // The remainder
        if upper_offset != lower_offset {
            let data_end_ptr = data_ptr;
            // The other OR is there to add 1 because upper_bound is inclusive
            let data_end = *data_end_ptr & (((1 << upper_bit_offset) - 1) | (1 << upper_bit_offset));
            counter += data_end.count_ones() as usize;
        }
        counter
    }

    pub fn first_one_bounds(&self, lower_bound: usize, upper_bound: usize) -> Option<usize> {
        if lower_bound >= self.bit_capacity {
            panic!("Bit index out of bounds");
        }
        if upper_bound > self.bit_capacity {
            panic!("Upper bound out of bounds");
        }
        let index = unsafe { self.first_one_unchecked(lower_bound) };
        if index >= upper_bound {
            None
        } else {
            Some(index)
        }
    }

    pub fn first_zero_bounds(&self, lower_bound: usize, upper_bound: usize) -> Option<usize> {
        if lower_bound >= self.bit_capacity {
            panic!("Bit index out of bounds");
        }
        if upper_bound > self.bit_capacity {
            panic!("Upper bound out of bounds");
        }
        let index = unsafe { self.first_zero_unchecked(lower_bound) };
        if index >= upper_bound {
            None
        } else {
            Some(index)
        }
    }

    pub fn first_zero(&self, bit_index: usize) -> Option<usize> {
        if bit_index >= self.bit_capacity {
            panic!("Bit index out of bounds");
        }

        let index = unsafe { self.first_zero_unchecked(bit_index) };
        if index >= self.bit_capacity {
            None
        } else {
            Some(index)
        }
    }
    pub fn first_one(&self, bit_index: usize) -> Option<usize> {
        if bit_index >= self.bit_capacity {
            panic!("Bit index out of bounds");
        }

        let index = unsafe { self.first_one_unchecked(bit_index) };
        if index >= self.bit_capacity {
            None
        } else {
            Some(index)
        }
    }
    /// Returns higher > bit_capacity in case of not found
    pub unsafe fn first_zero_unchecked(&self, bit_index: usize) -> usize {
        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & BIT_END_OFFSET;

        //We scan the last chunk with trailing, if it fails it shall return the max
        let last_chunk = self.data.add(self.capacity - 1);
        let mut data_ptr = self.data.add(offset);
        let mut counter = offset * (BIT_END_OFFSET + 1);
        let mut data = *data_ptr | ((1 << bit_offset) - 1);

        while data & BIT_MASK == BIT_MASK {
            if data_ptr != last_chunk {
                data_ptr = data_ptr.add(1);
                data = *data_ptr;
                counter += BIT_END_OFFSET + 1;
                continue;
            }
            break;
        }
        counter += data.trailing_ones() as usize;
        counter
    }

    /// Returns higher > bit_capacity in case of not found
    pub unsafe fn first_one_unchecked(&self, bit_index: usize) -> usize {
        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & BIT_END_OFFSET;

        //We scan the last chunk with trailing, if it fails it shall return the max
        let last_chunk = self.data.add(self.capacity - 1);
        let mut data_ptr = self.data.add(offset);
        let mut counter = offset * (BIT_END_OFFSET + 1);
        let mut data = *data_ptr & !((1 << bit_offset) - 1);

        while !data & BIT_MASK == BIT_MASK {
            if data_ptr != last_chunk {
                data_ptr = data_ptr.add(1);
                data = *data_ptr;
                counter += BIT_END_OFFSET + 1;
                continue;
            }
            break;
        }
        counter += data.trailing_zeros() as usize;
        counter
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
        let bit_offset = bit_index & BIT_END_OFFSET;
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
        let bit_offset = bit_index & (BIT_END_OFFSET);
        unsafe {
            let ptr = self.data.add(offset);
            Some((*ptr & (1 << bit_offset)) != 0)
        }
    }
    #[inline(always)]
    pub unsafe fn set_unchecked(&mut self, bit_index: usize, value: bool) {
        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (BIT_END_OFFSET);
        unsafe {
            let ptr = self.data.add(offset);
            *ptr = (*ptr & !(1 << bit_offset)) | ((value as usize) << bit_offset);
        }
    }
    #[inline(always)]
    pub unsafe fn get_unchecked(&self, bit_index: usize) -> bool {
        let offset = bit_index >> DIV_SHIFT;
        let bit_offset = bit_index & (BIT_END_OFFSET);
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
