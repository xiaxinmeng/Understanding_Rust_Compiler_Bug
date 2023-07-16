rust
pub fn eat_digits(s: &[u8]) -> (&[u8], &[u8]) {
    for i in 0..s.len(){
        if !s[i].is_ascii_digit(){
            return s.split_at(i);
        }
    }
    s.split_at(s.len())
}
