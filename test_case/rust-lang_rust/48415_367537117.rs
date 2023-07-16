rust
pub trait Foo {
  /// [x](::Bar)
 fn foo();
}
pub trait Bar {
  /// [x](::Foo)
  fn foo();
}
