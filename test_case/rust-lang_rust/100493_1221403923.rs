rust
fn foo<'a>(&'a self) -> &'a Foo {
  &self.inner
}
