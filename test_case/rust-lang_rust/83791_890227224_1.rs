rust
fn next(&mut self) -> Option<Self::Item> {
  Some((self.a.next()?, self.b.next()?))
}
