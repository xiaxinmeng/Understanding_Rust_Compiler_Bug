
    /// Remove the key-value pair at the given index
    pub fn remove_as_leaf(&mut self, index: uint) -> (K, V) {
        match (self.keys.remove(index), self.vals.remove(index)) {
            (Some(k), Some(v)) => (k, v),
            _ => unreachable!(),
        }
    }
