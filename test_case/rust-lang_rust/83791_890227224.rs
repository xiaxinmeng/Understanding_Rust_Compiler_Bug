rust
fn next(&mut self) -> Option<Self::Item> {
  match (self.a.next(), self.b.next()) {
     (Some(a), Some(b)) => Some((a, b))
     _ => None
  }
}
