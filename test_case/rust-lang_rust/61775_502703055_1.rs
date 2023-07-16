rust
fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> impl Foo<'a, 'b> { if something { a } else { b } }
