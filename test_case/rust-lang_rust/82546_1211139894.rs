rust
struct Foo<T>;

impl<T> Foo<T> {
    fn new() -> Foo<_> {
        Foo
    }
}
