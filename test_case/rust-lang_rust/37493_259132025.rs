 rust
    pub fn span_if_local(&self, id: DefId) -> Option<Span> {
        self.as_local_node_id(id).map(|id| self.span(id))
    }

    pub fn span_if_local_opt(&self, id: DefId) -> Option<Span> {
        self.as_local_node_id(id).and_then(|id| self.opt_span(id))
    }
