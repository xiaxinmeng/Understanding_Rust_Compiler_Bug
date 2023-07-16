rust
// Without `.unzip()`
impl<'a, T> Iterator for EdgeIter<'a, T> {
    type Item = &'a Edge<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let edges = self.edges.and_then(|edges| {
            if let [edge, rest @ ..] = edges {
                Some((rest, edge))
            } else {
                None
            }
        });
        self.edges = edges.map(|edges| edges.0);

        edges.map(|edges| edges.1).and_then(|next_id| {
            let next = self.edge_data.get(next_id);
            debug_assert!(next.is_some(), "edge data missing edge {}", next_id);

            next
        })
    }
}

// With `.unzip()`
impl<'a, T> Iterator for EdgeIter<'a, T> {
    type Item = &'a Edge<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (edges, next) = self
            .edges
            .and_then(|edges| {
                if let [edge, rest @ ..] = edges {
                    Some((rest, edge))
                } else {
                    None
                }
            })
            .unzip();
        self.edges = edges;

        next.and_then(|next_id| {
            let next = self.edge_data.get(next_id);
            debug_assert!(next.is_some(), "edge data missing edge {}", next_id);

            next
        })
    }
}
