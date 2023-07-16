rust
    pub fn is_closure(self, def_id: DefId) -> bool {
        matches!(self.def_kind(def_id), DefKind::Closure)
    }
