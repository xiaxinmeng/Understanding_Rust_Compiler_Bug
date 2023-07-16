 rust
struct Foo<T> { ... }

impl<T> Foo<T> {
    fn new<U>(u: U) -> Foo<U> {
        Self { ... } // is this Foo<T> or Foo<U>?
    }
}
