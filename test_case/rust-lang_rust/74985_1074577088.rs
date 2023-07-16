rust
impl<T> [T] {
    fn get_chunk<const N: usize>(&self, idx: usize) -> &[T; N] {
        &self[idx..idx + N].as_chunks::<N>().0[0]
    }
    fn get_chunk_mut<const N: usize>(&mut self, idx: usize) -> &mut [T; N] {
        &mut self[idx..idx + N].as_chunks_mut::<N>().0[0]
    }
}
