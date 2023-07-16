
    pure fn top(&self) -> &self/T { &self.data[0] }

    pure fn maybe_top(&self) -> Option<&self/T> {
        if self.is_empty() { None } else { Some(self.top()) }
    }
