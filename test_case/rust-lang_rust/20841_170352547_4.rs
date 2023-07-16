 rust
fn foo<F>(_: F) {}
fn id<T: Fn(f32) -> f32>(x: T) -> T {x}
foo(id(|x| x.sin()))
