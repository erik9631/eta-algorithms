use std::marker::PhantomData;

pub struct ArrayIterator<'a, T>
where
    T: Copy + Sized,
{
    #[allow(dead_code)]
    pub(crate) phantom_data: &'a PhantomData<T>,
    pub(crate)data: *mut T,
    pub(crate)end: *mut T,
}

pub struct ArrayIteratorMut<'a, T>
where
    T: Copy + Sized,
{
    #[allow(dead_code)]
    pub(crate)phantom_data: &'a mut PhantomData<T>,
    pub(crate)data: *mut T,
    pub(crate)end: *mut T,
}



macro_rules! impl_iterator {
    ($name:ident; $item:ty; $mutability:tt) => {
        impl<'a, T> Iterator for $name<'a, T>
        where
            T: Copy + Sized,
        {
            type Item = $item;
            fn next(&mut self) -> Option<Self::Item> {
                if self.data >= self.end {
                    return None
                }
                unsafe {
                    let item = (self.data).$mutability().unwrap();
                    self.data = self.data.offset(1);
                    Some(item)
                }

            }
        }
    };
}

impl_iterator!(ArrayIterator; &'a T; as_ref);
impl_iterator!(ArrayIteratorMut; &'a mut T; as_mut);