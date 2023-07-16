rust
/// Safety: `v.len() < v.capacity()`
unsafe fn test(v: &mut Vec<u32>, x: &mut u32) {
    v.push(7);
    *x = 8;
}
