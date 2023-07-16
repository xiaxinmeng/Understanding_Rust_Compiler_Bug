rust
impl<T> NonNull<[T]> {
    #[inline]
    fn as_mut_ptr(self) -> *mut T {
        self.as_non_null_ptr().as_ptr()
    }
}
