 rust
trait Foo {
    fn x(&self);
}

impl Foo for int {
    fn x(&self) {}
}

impl<F:Foo> Foo for @F {
    fn x(&self) {
        (*self).x()
    }
}

fn main() {
    (@1i).x()
}
