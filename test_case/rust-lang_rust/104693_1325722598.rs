rust
pub fn is_hex_mask(chunk: &[u8; 16]) -> bool {
    let x = u8x16::from_array(*chunk);
    let m1 = x.simd_gt(splat(b'0' - 1));
    let m2 = x.simd_lt(splat(b'9' + 1));
    let m3 = x.simd_gt(splat(b'a' - 1));
    let m4 = x.simd_lt(splat(b'f' + 1));
    let m = (m1 & m2) | (m3 & m4);
    m.all()
}
