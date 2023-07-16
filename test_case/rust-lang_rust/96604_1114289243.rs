rust
#[inline(always)]
fn is_definitely_zero2<const N: usize>(x: &[u8; N]) -> bool {
    let zero = is_zero(x);
    if optimized_to_const(&zero) {
        zero
    } else {
        false
    }
}
