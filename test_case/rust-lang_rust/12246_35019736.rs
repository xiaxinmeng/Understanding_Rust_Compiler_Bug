 rust
trait Foo {
    fn foo(&self);
}
pub fn foo(x: &Foo) {
    x.foo();
}
