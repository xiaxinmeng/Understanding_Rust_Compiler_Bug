rust
fn decrypt<const N: usize>(data: &mut [u8], key: &[u8; N]) {
    data.chunks_mut(N)
        .for_each(|chunk| {
            for (i, x) in chunk.iter_mut().enumerate() {
                *x ^= key[i];
            }
        });
}
