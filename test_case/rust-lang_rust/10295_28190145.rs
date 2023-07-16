 rust
trait Foo {}

impl Foo for [int] {}
impl Foo for int {}

fn main() {
     let v = (&[1i,2]) as &Foo;
     let w = &1i as &Foo;
}
