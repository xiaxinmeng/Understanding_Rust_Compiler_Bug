rust
#[no_mangle]
fn f(slice: &[u64], start: usize, end: usize) -> u64 {
    let mut total = 0;
    assert!(start < end && end <= slice.len());
    let end = std::cmp::min(slice.len(), end);
    for i in start..end {
        total += slice[i];
    }
    total
}
