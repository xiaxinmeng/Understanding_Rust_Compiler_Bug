 rust
pub trait Foo {
    type bar;
    fn bar(&self) {}
}
impl Foo for char {
    type bar = u8;
}
fn main() {
    Foo::bar(&'a');
    let _: <char as Foo>::bar = <char as Foo>::bar::max_value();
}
