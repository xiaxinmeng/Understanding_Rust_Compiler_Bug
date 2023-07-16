rust
impl<T: Debug> Option<T> {
    pub fn expect_none(self, msg: &str);
    pub fn unwrap_none(self);
}
