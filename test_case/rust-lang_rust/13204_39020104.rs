 rust
pub trait Foo {
  fn bar<'a, I: Iterator<&'a ()>>(&mut self, _: I) {}
}

pub struct Baz;

impl Foo for Baz {}

fn main() {}
