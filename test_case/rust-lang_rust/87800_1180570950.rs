rust
// core::option

impl<T, U> Option<(T, U)> {
    pub fn unzip(self) -> (Option<T>, Option<U>);
}
