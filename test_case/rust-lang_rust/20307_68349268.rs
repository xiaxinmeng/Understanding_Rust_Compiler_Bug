 rust
#![feature(associated_types)]

trait Foo {
  fn foo(&self) -> ();
}

trait Bar {
  type F: Foo;
}

struct Wat<B: Bar> {
  fs: Vec<B::F>,
}

impl<B: Bar> Wat<B> {
  fn wat(&mut self) -> () {
      Foo::foo(&self.fs.pop().unwrap()); // <-- changed to UFCS here
  }
}

fn main() {
}
