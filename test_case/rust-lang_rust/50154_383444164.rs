rust
#[inline(always)]
fn cmp_gt_i16x16(lhs: i16x16, rhs: i16x16) -> i16x16 {
    let lz = rhs - lhs;
    let sign_bit = lz & i16x16::splat(-32768);
    sign_bit >> 15
}
