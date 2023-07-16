rust
impl<T> MutexGuard<'_, T> {
    pub fn guard<U, F: FnOnce() -> U>(self, f: F) -> U {
        let val = f();
        drop(self);
        val
    }
}
