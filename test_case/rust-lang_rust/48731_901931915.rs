rust
fn slice_shift_slice(a: &[u8]) -> Option<(u8, &[u8])> {
    let mut iter = a.iter();
    iter.next().map(|&n| (n, iter.as_slice()))
}
