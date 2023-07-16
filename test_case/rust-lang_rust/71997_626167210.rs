rust
#[no_mangle]
fn f(slice: &[u64], start: usize, end: usize) -> u64 {
    let mut total = 0;
    assert!(start < end && start < slice.len() && end <= slice.len());
    for i in (start..end).take_while(|&i| i < slice.len()) {
        total += slice[i];
    }
    total
}
