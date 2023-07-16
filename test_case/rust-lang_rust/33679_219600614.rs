 rust
pub struct Foo<'a>(&'a str);

impl<'a> Iterator for Foo<'a> {
  type Item = &'a str;
  fn next(&mut self) -> Option<&'a str> {
    Some(self.0)
  }

  /// All your docs are
  ///
  /// belong to us
  fn size_hint(&self) -> (usize, Option<usize>) {
   (1,None)
  }
}
