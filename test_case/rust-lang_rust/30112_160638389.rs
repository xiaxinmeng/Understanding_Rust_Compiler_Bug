 rust
pub fn split_at(input: &[u8]) -> &[u8] {
    for i in 0..input.len() {
        if input[i] == b' ' {
            return &input[..i]
        }
    }
    &[]
}
