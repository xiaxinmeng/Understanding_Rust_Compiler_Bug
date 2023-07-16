 Rust
fn foo<T>(&T) -> impl Foo
-->
fn foo<'total, T: 'total>(&T) -> impl Foo + 'total
