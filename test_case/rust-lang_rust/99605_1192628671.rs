rust
pub trait Foo {
    fn foo(&self);
}

struct Bar;
impl Foo for Bar {
    #[track_caller]
    fn foo(&self) {}
}
