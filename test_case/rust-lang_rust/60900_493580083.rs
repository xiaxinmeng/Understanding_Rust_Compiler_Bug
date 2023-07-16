rust
trait A {
    fn a(&self) {}
}

trait B {
    fn b(&self) {}
}

trait Foo: A + B {}

impl A for () {}
impl B for () {}
impl Foo for () {}


fn main() {
    let x: &Foo = &();
    let y: &A = x;
}
