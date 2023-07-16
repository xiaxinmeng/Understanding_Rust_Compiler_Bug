 rust
impl<T> Carrier<Option<T>> for Option<T> {
    type Ok = T;
    fn is_ok(&self) -> bool { self.is_some() }
    fn unwrap_into_ok(self) -> Self::Ok { self.unwrap() }
    fn unwrap_into_error(self) -> { debug_assert!(self.is_none()); None }
}
