rust
    pub fn enclosing_body_owner(&self, hir_id: HirId) -> HirId {
        for (parent, _) in self.parent_iter(hir_id) {
            if let Some(body) = self.maybe_body_owned_by(parent) {
                return self.body_owner(body);
            }
        }

        bug!("no `enclosing_body_owner` for hir_id `{}`", hir_id);
    }
