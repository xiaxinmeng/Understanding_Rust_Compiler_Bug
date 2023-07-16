rust
TyImplTraitUniversal(fn_def_id, _) => {
    let impl_trait_def_id = self.tcx.hir.local_def_id(ast_ty.id);
    let generics = self.tcx.generics_of(fn_def_id);
    let index = /* find the index of `impl_trait_def_id` in `generics`, see LINK1 below */;
    self.tcx.mk_param(index, impl_trait_def_id)
}
