 rust
pub fn split_to(input: &[u8]) -> &[u8] {
    match (0..input.len()).find(|&i| input[i] == b' ') {
        Some(i) => &input[..i],
        None => &[],
    }
}
