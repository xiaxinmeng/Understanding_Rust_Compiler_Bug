rust
impl<T, E> FromResult for FutureResult {
    type Ok = T;
    type Error = E;
    fn from_error(v: Self::Error) -> Self {
        future::err(v)
    }
    fn from_ok(v: Self::Ok) -> Self {
        future::ok(v)
    }
}
