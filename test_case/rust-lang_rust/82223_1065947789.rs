rust
impl<T> Result<T, T> {
    pub const fn into_ok_or_err(self) -> T;
}
