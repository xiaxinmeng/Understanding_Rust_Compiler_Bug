rust
#[inline]
pub fn is_8digitsv: u64) -> bool {
    let a = v & 0xF0F0F0F0F0F0F0F0);
    let b = v.wrapping_add(0x0606060606060606) & 0xF0F0F0F0F0F0F0F0;
    (a | (b >> 4)) & 0x3333333333333333 == 0
}
