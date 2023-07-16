rust
impl<T> Cell<T> {
    pub fn with<U>(&self, func: const fn(&mut T) -> U) -> U;
}
