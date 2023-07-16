rust
// core::result

impl<T, E> Result<T, E> {
    pub fn inspect<F: FnOnce(&Self)>(self, f: F) -> Self;
}

// core::option

impl<T> Option<T> {
    pub fn inspect<F: FnOnce(&Self)>(self, f: F) -> Self;
}
