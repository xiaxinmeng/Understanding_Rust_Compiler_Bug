 rust
#![feature(associated_type_defaults)]

trait Foo {
  type Args = ();

  fn foo(&mut self, name: &str, args: Option<Self::Args>);
}

impl Foo for bool {
  fn foo(&mut self, name: &str, args: Option<Self::Args>) {}
}
