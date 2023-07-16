
#[allow(overflowing_literals)] // 0x8000_0000 is MIN_I32
pub fn flip_msb(a: i32) -> i32 {
    a ^ 0x8000_0000
}
