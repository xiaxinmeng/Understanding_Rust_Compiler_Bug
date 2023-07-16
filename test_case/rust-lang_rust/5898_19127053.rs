 rust
trait Foo {
    fn foo(&self);
}

trait Bar {
    fn bar(&self);
}

impl <T: Bar> Foo for T {
    fn foo(&self) { self.bar(); }
}
