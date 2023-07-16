rust
impl<T> Nullable for *mut T {
    const NULL: Self = 4usize as *mut _;

    #[inline]
    fn is_null(&self) -> bool {
        *self == Self::NULL
    }
}
