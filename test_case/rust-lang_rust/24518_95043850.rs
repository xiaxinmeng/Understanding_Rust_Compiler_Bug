 rust
    pub fn map3<F>(mut self, f: F) -> P<T> where
        F: FnOnce(&mut T),
    {
        f(&mut *self.ptr);
        self
    }
