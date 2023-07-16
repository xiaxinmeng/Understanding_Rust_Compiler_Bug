rust
match tcx.hir.body_owner_kind(self.mir_node_id) {
    BodyOwnerKind::Fn => {
        let closure_base_def_id = tcx.closure_base_def_id(self.mir_def_id);

        let defining_ty = if self.mir_def_id == closure_base_def_id {
            tcx.type_of(closure_base_def_id)
        } else {
            let tables = tcx.typeck_tables_of(self.mir_def_id);
            tables.node_id_to_type(self.mir_hir_id)
        };

        let defining_ty = self.infcx.replace_free_regions_with_nll_infer_vars(FR, &defining_ty);

        match defining_ty.sty {
                ty::TyClosure(def_id, substs) => DefiningTy::Closure(def_id, substs),
                ty::TyGenerator(def_id, substs, interior) => {
                    DefiningTy::Generator(def_id, substs, interior)
                }
                ty::TyFnDef(def_id, substs) => DefiningTy::FnDef(def_id, substs),
                _ => span_bug!(
                    tcx.def_span(self.mir_def_id),
                    "expected defining type for `{:?}`: `{:?}`",
                    self.mir_def_id,
                    defining_ty
                ),
        }
    }

    BodyOwnerKind::Const | BodyOwnerKind::Static(..) => {
        // new code will go here, see below
    }
}
