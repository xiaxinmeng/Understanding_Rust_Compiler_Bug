rust
struct Foo<'a, 'b> { f: &'a &'b u32 }
fn foo<'x, 'y>(foo: Foo<'x, 'y>) -> &'x u32 { foo.f }
