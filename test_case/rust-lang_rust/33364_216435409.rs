 rust
impl<'a, T: Foo<'a>> Wrap<T> {
  fn consume<F>(f: F) where F: Fn(<T as Foo<'a>>::Item) {
    self.foo.consume(f);
  }
}
