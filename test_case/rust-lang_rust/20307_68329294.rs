
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
    self.fs.pop().unwrap().foo();
  }
}

fn main() {
}
