 rust
struct Foo<T> { a: T }

impl Drop for Foo<int> {
    fn drop(&mut self) {}
}
