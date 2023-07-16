rust
#[no_mangle]
fn f(slice: &[u64], start: usize, mut end: usize) -> u64 {
    let mut total = 0;
    assert!(start < end && start < slice.len() && end <= slice.len());
    if end > slice.len() { end = slice.len(); }
    for i in start..end {
        total += slice[i];
    }
    total
}
