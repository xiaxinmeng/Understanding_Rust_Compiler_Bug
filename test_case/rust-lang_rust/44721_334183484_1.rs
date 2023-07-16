rust
TyKind::ImplTrait(ref bounds) => {
    match context.impl_trait_treatment() {
        ImplTraitTreatment::Existential => hir::TyImplTrait(self.lower_bounds(bounds)),
        ImplTraitTreatment::Universal | ImplTraitTreatment::Disallowed => {
            // For now, treat Universal as the same as disallowed since it's not done yet.
            span_err!(tcx.sess, ast_ty.span, E0562,
                              "`impl Trait` not allowed outside of function \
                               and inherent method return types");
            hir::TyErr
        }
    }
}
