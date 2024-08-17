pub struct FatPtrMut<T> {
    pub ptr: *mut T,
    pub end: *mut T,
}

impl<T> FatPtrMut<T> {
    const SIZE: usize = std::mem::size_of::<T>();
    #[inline(always)]
    pub fn new(ptr: *mut T, end: *mut T) -> Self {
        Self {
            ptr,
            end,
        }
    }
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        let bytes_len = self.end as usize - self.ptr as usize;
        unsafe { std::slice::from_raw_parts(self.ptr, bytes_len / Self::SIZE) }
    }
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let bytes_len = self.end as usize - self.ptr as usize;
        unsafe { std::slice::from_raw_parts_mut(self.ptr, bytes_len / Self::SIZE) }
    }

    pub fn from_mut_slice(slice: &mut [T]) -> Self {
        let ptr = slice.as_mut_ptr();
        let end = unsafe { ptr.add(slice.len()) };
        Self {
            ptr,
            end,
        }
    }

}


pub struct FatPtr<T> {
    pub ptr: *const T,
    pub end: *const T,
}

impl<T> FatPtr<T> {
    const SIZE: usize = std::mem::size_of::<T>();
    #[inline(always)]
    pub fn new(ptr: *const T, end: *const T) -> Self {
        Self {
            ptr,
            end,
        }
    }
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        let bytes_len = self.end as usize - self.ptr as usize;
        unsafe { std::slice::from_raw_parts(self.ptr, bytes_len / Self::SIZE) }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        let ptr = slice.as_ptr();
        let end = unsafe { ptr.add(slice.len()) };
        Self {
            ptr,
            end,
        }
    }
}

impl<T> Iterator for FatPtr<T>
where
    T: 'static
{
    type Item = &'static T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr >= self.end {
            return None
        }
        unsafe {
            let item = self.ptr.as_ref().unwrap();
            self.ptr = self.ptr.offset(1);
            Some(item)
        }
    }
}

impl<T> Iterator for FatPtrMut<T>
where
    T: 'static
{
    type Item = &'static mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr >= self.end {
            return None
        }
        unsafe {
            let item = self.ptr.as_mut().unwrap();
            self.ptr = self.ptr.offset(1);
            Some(item)
        }
    }
}