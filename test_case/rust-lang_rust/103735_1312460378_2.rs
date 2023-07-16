rust
pub struct Test {
    data: [u8],
}

pub fn test_len(t: *const Test) -> usize {
    unsafe { (*t).data.len() }
}
