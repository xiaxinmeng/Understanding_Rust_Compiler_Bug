
struct Foo {
  field: i32
}

impl Foo {
  fn foo<'a>(&self, x: &'a Foo) -> &'a Foo {
    if true { self } else { x }
  }
}

fn main() { }
