rust
trait Foo {
    fn a();
    fn b(&self);
}

impl Foo for i32 {
    fn a() {}
    fn b(&self) {}
}
