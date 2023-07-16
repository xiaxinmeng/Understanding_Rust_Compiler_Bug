rust
fn next(&mut self) -> Option<Self::Item> {
    if let Some(edges) = self.edges {
        if let [next_id, rest @ ..] = edges {
            self.edges = Some(rest);
            let next = self.edge_data.get(next_id);
            debug_assert!(next.is_some(), "edge data missing edge {}", next_id);
            return next;
        }
    }
    None
}
