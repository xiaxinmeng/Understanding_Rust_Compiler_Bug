rust
impl<T: ?Sized> Arc<T> {
    pub unsafe fn clone_from_raw(ptr: *const T) -> Arc<T> {…}
}
