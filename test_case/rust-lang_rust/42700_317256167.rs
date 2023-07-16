rust

struct Foo {
  field: i32
}

impl Foo {
  fn foo<'a>(&self, mut x: Vec<&'a Foo>) {
    x.push(self);
  }
}

fn main() { }
