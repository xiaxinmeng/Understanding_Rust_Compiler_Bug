 rust
struct MyBool(bool);
impl<T> Carrier<MyBool> for MyBool {
    type Ok = ();
    fn is_ok(&self) -> bool { self.0 }
    fn unwrap_into_ok(self) -> Self::Ok { debug_assert!(self.0); () }
    fn unwrap_into_error(self) -> { debug_assert!(!self.0); self }
}
