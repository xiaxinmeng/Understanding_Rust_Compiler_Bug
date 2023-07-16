 rust
pub trait Foo {
    fn bar(&self) {}
    type bar;
}
impl Foo for char {
    type bar = u8;
}
fn main() {}
