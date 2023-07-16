rust
pub fn is_char_boundary(s: &str, index: usize) -> bool {
    match s.as_bytes().get(index) {
        None if index == s.len() => true,
        None => false,
        Some(_) if index == 0 => true,
        Some(&b) => (b as i8) >= -0x40,
    }
}
