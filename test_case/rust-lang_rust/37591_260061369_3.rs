 rust
impl<T, E> Future<Item=T, Error=E> {
    pub fn finished(e: E) -> Self { ... }
    pub fn failed(e: E) -> Self { ... }
    pub fn done(e: E) -> Self { ... }
}
