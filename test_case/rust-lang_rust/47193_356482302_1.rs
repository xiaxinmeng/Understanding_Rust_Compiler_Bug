rust
impl<T, E> Result<Option<T>, E> {
    pub fn transpose(self) -> Option<Result<T, E>>;
}

impl<T, E> Option<Result<T, E>> {
    pub fn transpose(self) -> Result<Option<T>, E>;
}
