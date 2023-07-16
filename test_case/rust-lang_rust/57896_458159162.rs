rust
fn foo<T>() -> Foo<T, T> { ... }
fn bar<T, U>() -> Foo<T, U> { ... }
fn mep<U, T>() -> Foo<T, U> { ... }
