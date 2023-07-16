 rust
impl<R> Clone for fn() -> R {
    fn clone(&self) -> Self { self }
}
impl<R, A1> Clone for fn(A1) -> R {
    fn clone(&self) -> Self { self }
}
impl<R, A1, A2> Clone for fn(A1, A2) -> R {
    fn clone(&self) -> Self { self }
}
impl<R, A1, A2, A3> Clone for fn(A1, A2, A3) -> R {
    fn clone(&self) -> Self { self }
}
