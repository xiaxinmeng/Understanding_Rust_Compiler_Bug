 rust
fn foo<'a>(x: &'a u32, y: &'a u32) -> &'a u32 { bar(x, y) }
fn bar<'b, 'c>(x: &'b u32, y: &'c u32) -> &'b u32 { foo(x, y) } //~ ERROR E0495
