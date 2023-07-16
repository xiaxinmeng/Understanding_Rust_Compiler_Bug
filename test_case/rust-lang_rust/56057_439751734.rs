rust
pub fn index(table: &[u128; 4], idx: usize) -> u128 {
    table[(idx & 0b11_0000) >> 4]
}
