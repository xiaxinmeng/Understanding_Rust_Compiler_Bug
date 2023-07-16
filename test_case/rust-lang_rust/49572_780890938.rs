rust
pub fn non_zero(mut x: NonZeroU32) -> bool {
    non_zero_ind(&x)
}

fn non_zero_ind(x: &NonZeroU32) -> bool {
    (*x).get() != 0
}
