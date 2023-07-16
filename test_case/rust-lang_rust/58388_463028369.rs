rust
pub fn foo(bytes: &[u8]) -> u8 {
    let (left, right) = bytes.split_at(bytes.len() >> 1);
    let right = &right[..left.len()]; // <--
    let mut total: u8 = 0;
    for i in 0..left.len() {
        let a = left[i];
        let b = right[i];
        let n = a.wrapping_add(b);
        total = total.wrapping_add(n);
    }
    total
}
