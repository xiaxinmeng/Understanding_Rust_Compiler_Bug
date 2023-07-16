rust
impl<T> Default for MaybeUninit<T> {
    fn default() -> Self {
        Self::uninit()
    }
}
