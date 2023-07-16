rust
impl<T: Clone, const N: usize> Clone for [T; N] {
    #[inline]
    fn clone(&self) -> Self {
        core::intrinsics::clone_array(self)
    }
}
