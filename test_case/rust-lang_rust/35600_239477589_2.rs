 rust
pub trait Foo {
    type bar;
    fn bar(&self) {}
}
impl Foo for char {
    type bar = u8;
}
fn main() {
    'a'.bar();
}
