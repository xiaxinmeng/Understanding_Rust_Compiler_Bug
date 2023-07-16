rust
struct Foo<const M: u8, const M: u8 = M>();
fn a(x: Foo<0>) {}
fn main() {}
