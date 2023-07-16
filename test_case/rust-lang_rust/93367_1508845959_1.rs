rust
  unsafe impl<'a, G: Group> Send for View<'a, G> where G::Container: Send {}
  