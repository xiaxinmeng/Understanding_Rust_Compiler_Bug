rust
trait Foo {
    fn bar(&self);
}
impl Foo for () {
    fn bar(&self) {}
}
const D: &Foo = &();
