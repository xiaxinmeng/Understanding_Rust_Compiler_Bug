rust
pub trait ArrayHelper<T, const N: usize> {
    fn split_array<const M: usize>(self) -> ([T; M], [T; N - M]);
    fn split_array_ref<const M: usize>(&self) -> (&[T; M], &[T; N - M]);
    fn split_array_mut<const M: usize>(&mut self) -> (&mut [T; M], &mut [T; N - M]);
}
