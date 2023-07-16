 rust
struct Foo<'s> { x: &'s i32 }
fn foo(x: &Foo<'static>) -> &Foo<'static> { x }
fn main() { }
