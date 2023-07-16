 rust
struct Foo<'self> {
  a: &'self int
}

impl<'self> Foo<'self> {
  fn foo(&mut self) {}
}

fn main() {
  let n = 1;
  let mut foo = Foo { a: &n };
  foo.foo();
  foo.foo();
}
