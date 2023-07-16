 rust
trait Carrier<Target> {
    type Ok;
    fn is_ok(&self) -> bool; // if true, represents the "ok-like" variant
    fn unwrap_into_ok(self) -> Self::Ok; // may panic if not ok
    fn unwrap_into_error(self) -> Target; // may panic if not error
}
