rust
impl<T, const N: usize> [T; N] {
    pub fn split_array_ref<const M: usize>(&self) -> Option<(&[T; M], &[T])>;
    pub fn split_array_mut<const M: usize>(&mut self) -> Option<(&mut [T; M], &mut [T])>;

    pub fn rsplit_array_ref<const M: usize>(&self) -> Option<(&[T], &[T; M])>;
    pub fn rsplit_array_mut<const M: usize>(&mut self) -> Option<(&mut [T], &mut [T; M])>;
}
