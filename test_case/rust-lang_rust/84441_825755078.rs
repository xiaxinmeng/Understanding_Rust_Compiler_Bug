rust
pub fn zero(s: &mut [u32], len: usize) -> &[u32] {
    let s = &mut s[..len];
    s.iter_mut().for_each(|x| *x = 0);
    s
}
