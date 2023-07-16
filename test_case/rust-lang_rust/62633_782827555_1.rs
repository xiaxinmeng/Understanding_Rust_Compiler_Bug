rust
impl<T: fmt::Debug, E> Result<T, E> {
    pub fn expect_err(self, msg: &str) -> E;
    pub fn unwrap_err(self) -> E;
}
