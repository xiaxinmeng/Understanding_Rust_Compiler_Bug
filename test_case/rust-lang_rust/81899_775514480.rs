rust
const CACHE: &[u32] = &const_sort(&[2, 3, 1], |a, b| a < b); // that line is a failure reason
const fn const_sort<F, const N: usize>(v: &[u32; N], mut is_less: F) -> [u32; N]
where
    F: FnMut(&u32, &u32) -> bool,
{
    todo!()
}
