 rust
struct Baz<T>(T);
impl<T> Foo for Baz<T> {
    type Output = fn(T);
}
