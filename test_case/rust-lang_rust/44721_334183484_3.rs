rust
fn visit_ty(&mut self, ty: &hir::Ty) {
    if let hir::TyTraitUniversal(..) = ty.node {
        self.implicit_defs.push(
            ty::TypeParameterDef {
                index: /* compute next index -- this should come after the generics the user wrote, presumably */,
                name: /* um what should we put here? */,
                def_id: tcx.hir.local_def_id(ty.id), // <-- use def-id we created for the `impl Trait` instance
                has_default: false,
                object_lifetime_default: rl::Set1::Empty,
                pure_wrt_drop: false,
                synthetic: Some(...), // <-- this is synthetic, naturally
        });
    }

    intravisit::walk_ty(ty);
}
