rust
struct Foo<'a> { x: &'a u32 }
fn foo<'x>(_: Foo<'x>) { }
