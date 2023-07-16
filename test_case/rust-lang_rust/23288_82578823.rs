 rust
impl<T, R: As<[T]>> PartialEq<R> for [T] {
    fn eq(&self, other: &R) -> bool { ... }
}
