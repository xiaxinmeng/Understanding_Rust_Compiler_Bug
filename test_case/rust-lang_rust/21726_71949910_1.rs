 rust
struct Bar<T>(T); // covariant in T
impl<T> Foo for Bar<T> {
    type Output = fn() -> T;
}
