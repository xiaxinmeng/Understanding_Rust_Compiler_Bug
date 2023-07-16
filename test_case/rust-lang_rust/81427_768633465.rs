rust
pub fn eat_digits(s: &[u8]) -> (&[u8], &[u8]) {
    let pos = s.iter().position(|c| !c.is_ascii_digit()).unwrap_or(s.len());
    s.split_at(pos)
}
