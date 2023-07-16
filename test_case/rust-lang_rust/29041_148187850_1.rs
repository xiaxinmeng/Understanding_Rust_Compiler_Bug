 rust
unsafe impl<T: Send + Clone> Sync for AtomicCell<T> {}
