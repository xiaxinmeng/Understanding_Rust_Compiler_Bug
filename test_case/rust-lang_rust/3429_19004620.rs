 rust
fn main() {
    let x = X;
    x.foobar();
}

trait Foo {
    fn foobar(&self);
}

trait Bar {
    fn foobar(&self);
}

trait FooBase {}

trait BarBase {}

impl<T: FooBase> Foo for T {
    fn foobar(&self) {}
}

impl<T: BarBase> Bar for T {
    fn foobar(&self) {}
}

struct X;

impl FooBase for X {}
