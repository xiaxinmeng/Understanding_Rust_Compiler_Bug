Rust
trait Foo {
    fn foo(self: Rc<Self>);
}
impl Foo for () {
    fn foo(self: Rc<Self>) {}
}
