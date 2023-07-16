rust
pub fn invalid_bitcast(value: i32x16) -> i32x16 {
    value.clamp(i32x16::splat(-8), i32x16::splat(8))
}
