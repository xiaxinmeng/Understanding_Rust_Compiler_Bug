
impl<T, const N: usize> [T; N] {
   pub fn try_split_at(&self, mid: usize) -> Option<&[T], &[T]>

    pub fn try_split_array_ref<const M: usize>(&self) -> Option<(&[T; M], &[T])>;
    pub fn try_split_array_mut<const M: usize>(&mut self) -> Option<(&mut [T; M], &mut [T])>;

    pub fn try_rsplit_array_ref<const M: usize>(&self) -> Option<(&[T], &[T; M])>;
    pub fn try_rsplit_array_mut<const M: usize>(&mut self) -> Option<(&mut [T], &mut [T; M])>;
}
