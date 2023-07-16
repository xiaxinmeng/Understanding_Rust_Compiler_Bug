rust
fn get_many_check_valid<const N: usize>(indices: &[usize; N], len: usize) -> bool {
    let mut valid = true;
    if N <= 10 {
        // NB: The optimzer should inline the loops into a sequence
        // of instructions without additional branching.
        for (i, &idx) in indices.iter().enumerate() {
            valid &= idx < len;
            for &idx2 in &indices[..i] {
                valid &= idx != idx2;
            }
        }
    } else {
        let mut sorted = *indices;
        sorted.sort_unstable();
        for i in 1..N {
            valid &= sorted[i-1] < sorted[i];
        }
        valid &= sorted[N-1] < len;
    }

    valid
}
