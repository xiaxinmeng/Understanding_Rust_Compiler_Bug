 rust
use std::mem;

struct Foo;

impl Foo {
  fn something(&self) -> u32 {
    mem::transmute(self.non_existent[0])
  }
}

fn main() {
  let foo = Foo;
  foo.something();
}
