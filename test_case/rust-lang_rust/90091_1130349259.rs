rust
impl<T, const N: usize> [T; N] {
    pub fn split_array<const M: usize>(self) -> ([T; M], [T; N - M]);

    pub fn split_array_ref<const M: usize>(&self) -> (&[T; M], &[T; N - M]);
    pub fn split_array_mut<const M: usize>(&mut self) -> (&mut [T; M], &mut [T; N - M]);

    pub fn rsplit_array_ref<const M: usize>(&self) -> (&[T; N - M], &[T; M]);
    pub fn rsplit_array_mut<const M: usize>(&mut self) -> (&mut [T; N - M], &mut [T; M]);
}

impl<T> [T] {
    pub fn split_array_ref<const N: usize>(&self) -> Option<(&[T; N], &[T])>;
    pub fn split_array_mut<const N: usize>(&mut self) -> Option<(&mut [T; N], &mut [T])>;

    pub fn rsplit_array_ref<const N: usize>(&self) -> Option<(&[T], &[T; N])>;
    pub fn rsplit_array_mut<const N: usize>(&mut self) -> Option<(&mut [T], &mut [T; N])>;
}
