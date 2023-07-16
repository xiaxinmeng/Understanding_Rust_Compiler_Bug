
pub trait Foo {}

pub struct Struct;

impl Foo for Struct {}

fn func(thing: ~Foo) {}

fn main() {
  let a = ~Struct as ~Foo;
  let b = func(a);
}
