rust
impl<'a, 'b> PlaceholderExpander<'a, 'b> {
    fn remove(&mut self, id: ast::NodeId) -> Expansion {
        self.expansions.remove(&id).unwrap()
    }
}
