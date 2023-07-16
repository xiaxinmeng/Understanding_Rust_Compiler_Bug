rust
    pub fn for_each_relevant_impl<F: FnMut(DefId)>(
        self,
        def_id: DefId,
        self_ty: Ty<'tcx>,
        mut f: F,
    ) {
        let _: Option<()> = self.find_map_relevant_impl(def_id, self_ty, |did| {
            f(did);
            None
        });
    }
