
// EMIT_MIR lower_slice_len.array_bound.SimplifyLocals.diff
pub fn array_bound<const N: usize>(index: usize, slice: &[u8; N]) -> u8 {
    if index < slice.len() {
        slice[index]
    } else {
        42
    }
}
