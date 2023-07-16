rust
#[repr(u8)]
pub enum Exception {
    Low = 5,
    High = 10,
}

pub fn access(array: &[usize; 12], exc: Exception) -> usize {
    array[(exc as u8 - 4) as usize]
}
