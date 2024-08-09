#[inline(always)]
pub fn rotate_inc(val: usize, mask: usize) -> usize{
    return (val + 1) & mask;
}

#[inline(always)]
pub fn rotate_dec(val: usize, mask: usize) -> usize{
    return (val - 1) & mask;
}

#[inline(always)]
pub fn closest_pow2(mut n: usize) -> usize {
    n -= 1; // In case we have a power of 2
    // This is for setting all the bits to the right to 1

    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    return n + 1
}