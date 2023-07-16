`rust
fn foo<T>(_: T) -> impl Send { }
fn bar<'a>(x: &'a ()) -> impl Send { foo(x) }
