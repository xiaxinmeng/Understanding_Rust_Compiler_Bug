Rust
    pub fn signature<'a, 'tcx>(&self, tcx: &TyCtxt<'a, 'tcx, 'tcx>) -> String {
        match self.kind {
            ty::AssociatedKind::Method => {
                format!("{}",
                    // We skip the binder here because the binder would deanonymize all
                    // late-bound regions, and we don't want method signatures to show up
                    // `as for<'r> fn(&'r MyType)`. Pretty-printing handles late-bound
                    // regions just fine, showing `fn(&MyType)`.
                    tcx.type_of(self.def_id).fn_sig().skip_binder())
            }
            ty::AssociatedKind::Type => format!("type {};", self.name.to_string()),
            ty::AssociatedKind::Const => {
                format!("const {}: {:?};", self.name.to_string(), tcx.type_of(self.def_id))
            }
        }
+    }
