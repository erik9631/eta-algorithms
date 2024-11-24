use crate::data_structs::array::Array;
use crate::data_structs::bitmap::consts::{DIV_SHIFT, MASK};
use std::collections::HashMap;
use std::ops::BitOrAssign;

#[derive(Copy, Clone)]
pub struct Handle {
    pub bit_mask: usize,
    pub chunk: u8,
}

impl BitOrAssign for Handle {
    fn bitor_assign(&mut self, rhs: Self) {
        self.chunk = rhs.chunk;
        self.bit_mask |= rhs.bit_mask;
    }
}

impl Handle {
    pub fn new_batch(offsets: &[usize]) -> Array<Self> {
        let mut array = Array::new_default_bytes(offsets.len(), 0);
        let mut len = 0;
        let mut chunk_table = HashMap::<u8, usize>::new();
        for offset in offsets {
            let chunk_offset = (offset >> DIV_SHIFT) as u8;
            let index = match chunk_table.get(&chunk_offset) {
                Some(index) => *index,
                None => {
                    let index = len;
                    chunk_table.insert(chunk_offset, index);
                    len += 1;
                    index
                }
            };
            let bit_offset = offset & MASK;
            let mask: usize = 1 << bit_offset;
            unsafe {
                *array.index_unchecked_mut(index) |= Self {
                    bit_mask: mask,
                    chunk: chunk_offset,
                }
            };
        }
        array.resize(len);
        array
    }
}
