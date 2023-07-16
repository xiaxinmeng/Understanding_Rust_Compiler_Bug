rust
#[non_exhaustive]
enum Foo {
  A(bool),
  B
}
match foo {
  Foo::A(true) => {}
  Foo::B => {}
  _ => {}
}
