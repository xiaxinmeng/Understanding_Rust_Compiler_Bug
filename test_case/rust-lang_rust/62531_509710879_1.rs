
#![feature(float_to_from_bytes)]
#![feature(stmt_expr_attributes)]
pub fn compare() -> bool {
    let gf_u = 12.5f32;
    #[cfg(target_endian = "big")]
    return gf_u.to_ne_bytes() == [0x41, 0x48, 0x00, 0x00];
    #[cfg(not(target_endian = "big"))]
    return gf_u.to_ne_bytes() == [0x00, 0x00, 0x48, 0x41];
}
