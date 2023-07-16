rust
struct Foo;

impl Foo {
  fn foo(&mut self, _: f32) -> i32 { todo!() }
  fn bar(&mut self) -> f32 { todo!() }
  fn baz(&mut self) {
    Self::foo(&mut self, Self::bar(&mut self));
  }
}
