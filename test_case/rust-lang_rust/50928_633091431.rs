Rust
    #[inline]
    pub fn local_def_id_to_hir_id(&self, id: LocalDefId) -> hir::HirId {
        let node_id = self.def_id_to_node_id[id];
        self.node_id_to_hir_id[node_id].unwrap()
    }

    #[inline]
    pub fn opt_local_def_id_to_hir_id(&self, id: LocalDefId) -> Option<hir::HirId> {
        let node_id = self.def_id_to_node_id[id];
        self.node_id_to_hir_id[node_id]
    }
