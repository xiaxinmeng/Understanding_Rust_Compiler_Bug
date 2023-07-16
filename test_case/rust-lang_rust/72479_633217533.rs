rust
unsafe impl<T> Covariant<T> for Arc<T> {}
unsafe impl<T> Covariant<T> for Weak<T> {}
