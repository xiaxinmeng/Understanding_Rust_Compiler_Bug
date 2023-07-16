rust
impl<T: ?Sized> Arc<T> {
    pub unsafe fn incr_strong_count(ptr: *const T);
    pub unsafe fn decr_strong_count(ptr: *const T);
}
