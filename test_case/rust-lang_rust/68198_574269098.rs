rust
impl<T, F: FnOnce() -> T> DerefMut for Lazy<T, F>
