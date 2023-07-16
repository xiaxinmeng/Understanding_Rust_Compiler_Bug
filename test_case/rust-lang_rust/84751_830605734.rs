rust
pub fn is_char_boundary(s: &str, index: usize) -> bool {
    if index == 0 {
        true
    } else if index < s.len() {
        unsafe { *s.as_bytes().get_unchecked(index) as i8 >= -0x40 }
    } else if index == s.len() {
        true
    } else {
        false
    }
}

