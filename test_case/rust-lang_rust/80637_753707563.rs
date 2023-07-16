rust
let typeck_results = self.infcx.in_progress_typeck_results?;
let ty = typeck_results.borrow().node_type_opt(hir_id)?;
let ty = self.infcx.resolve_vars_if_possible(ty);
for inner in ty.walk() {
    if inner == self.target {
        return Some(ty);
    }
    if let GenericArgKind::Type(inner_ty) = inner.unpack() {
        if let ty::Infer(ty::TyVar(a_vid)) = inner_ty.kind() {
            if let GenericArgKind::Type(target_ty) = self.target.unpack() {
                if let ty::Infer(ty::TyVar(b_vid)) = target_ty.kind() {
                    if self.infcx.inner.borrow_mut().type_variables().sub_unified(a_vid, b_vid) {
                        return Some(ty);
                    }
                }
            }
        }
    }
}
None
