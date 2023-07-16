rust
let is_impl_trait_ref = tcx.impl_trait_ref(tcx.hir.local_def_id(item.id));
is_impl_trait = Some(ty::TraitRef {
    def_id: is_impl_trait_ref.unwrap().def_id,
    substs: Substs::identity_for_item(tcx, is_impl_trait_ref.unwrap().def_id)
});
