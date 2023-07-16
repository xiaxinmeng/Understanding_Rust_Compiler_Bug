rust
impl<T> Arc<T> {
    pub fn try_new_cyclic<F, E>(data_fn: F) -> T) -> Result<Arc<T>, E>
    where
        F: FnOnce(&Weak<T>) -> Result<T, E>,
    {
        // ...
    }
}
