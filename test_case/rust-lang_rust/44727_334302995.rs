rust
abstract type Foo<U>: impl Iterator<Item = U>;
fn foo<T>() -> Foo<T> { ... }
