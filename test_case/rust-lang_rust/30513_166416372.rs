 rust
impl<T> RecoverSafe for &AssertRecoverSafe<T> {}
impl<T> RecoverSafe for Rc<AssertRecoverSafe<T>> {}
// ...
