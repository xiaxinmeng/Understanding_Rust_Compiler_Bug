 rust
pub fn nodes(&self) -> Keys<'a, &T, HashSet<&T>> {
  self.edges.keys()
}
