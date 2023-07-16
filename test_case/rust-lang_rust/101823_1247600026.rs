rust
  fn candidate_method_names(&self) -> Vec<Ident> {
        let mut set = FxHashSet::default();
        let mut names: Vec<_> = self
            .inherent_candidates
            .iter()
            .chain(&self.extension_candidates)
            .filter(|candidate| {
                if let Some(return_ty) = self.return_type {
                    self.matches_return_type(&candidate.item, None, return_ty)
                } else {
                    true
                }
            })
            .map(|candidate| candidate.item.ident(self.tcx))
            .filter(|&name| set.insert(name))
            .collect();

        // Sort them by the name so we have a stable result.
        names.sort_by(|a, b| a.as_str().partial_cmp(b.as_str()).unwrap());
        names
    }

