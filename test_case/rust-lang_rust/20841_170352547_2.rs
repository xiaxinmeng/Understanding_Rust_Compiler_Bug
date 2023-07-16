 rust
fn foo<F: Fn(f32) -> f32>(_: F) {}
fn id<T>(x: T) -> T {x}
foo(id(|x| x.sin()))
