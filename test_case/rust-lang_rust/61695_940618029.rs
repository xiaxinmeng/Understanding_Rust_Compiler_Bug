rust
impl<T, E: Into<T>> Result<T, E> {
    pub fn into_ok(self) -> T {
        match self {
            Ok(x) => x,
            Err(e) => e.into(),
        }
    }
}
/// and a corresponding `into_err`
