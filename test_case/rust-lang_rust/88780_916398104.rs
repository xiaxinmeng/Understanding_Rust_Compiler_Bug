rust
pub fn abs_diff(slf: u8, other: u8)  -> u8 {
    (slf as i32).wrapping_sub(other as i32).abs() as u8
}
