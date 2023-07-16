 rust
pub unsafe fn vbsl_s8(a: uint8x8_t, b: int8x8_t, c: int8x8_t) -> int8x8_t {
    let not = int8x8_t(-1, -1, -1, -1, -1, -1, -1, -1);
    transmute(simd_or(
        simd_and(a, transmute(b)),
        simd_and(simd_xor(a, transmute(not)), transmute(c)),
    ))
}
