rust
    /// If the given `DefId` belongs to a trait that was automatically derived, returns `true`.
    pub fn is_builtin_derive(self, def_id: DefId) -> bool {
        self.has_attr(def_id, sym::automatically_derived)
    }
