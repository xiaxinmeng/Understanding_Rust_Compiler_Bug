rust
    fn node_ty_contains_target(&mut self, hir_id: HirId) -> Option<Ty<'tcx>> {
        self.infcx
            .in_progress_typeck_results
            .and_then(|typeck_results| typeck_results.borrow().node_type_opt(hir_id))
            .map(|ty| self.infcx.resolve_vars_if_possible(ty))
            .filter(|ty| {
                ty.walk().any(|inner| {
                    if inner == self.target {
                        return true;
                    }

                    let avid = Some(inner)
                        .if_let(GenericArgKind::Type(inner_ty), |inner| inner.unpack())
                        .if_let(ty::Infer(ty::TyVar(a_vid)), |inner_ty| inner_ty.kind());
                    let bvid = Some(self.target)
                        .if_let(GenericArgKind::Type(target_ty), |target| target.unpack())
                        .if_let(ty::Infer(ty::TyVar(b_vid)), |target_ty| target_ty.kind());

                    avid.zip(bvid).map_or(false, |(avid, bvid)| {
                        self.infcx.inner.borrow_mut().type_variables().sub_unified(a_vid, b_vid)
                    })
                })
            })
    }
