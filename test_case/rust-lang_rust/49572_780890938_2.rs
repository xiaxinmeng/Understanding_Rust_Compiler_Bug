rust
pub fn non_zero(mut x: NonZeroU32) -> u32 {
    non_zero_ind(&x)
}

pub fn non_zero_ind(x: &NonZeroU32) -> u32 {
    (*x).get().leading_zeros()
}
