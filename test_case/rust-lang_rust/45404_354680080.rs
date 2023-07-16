
ItemImpl(_, _, defaultness, ref generics, ..) => {
    if defaultness.is_default() {
        is_impl_trait_ref = tcx.impl_trait_ref(tcx.hir.local_def_id(item.id));
        let trait_ref_node = tcx.hir.get(tcx.hir.as_local_node_id(is_impl_trait_ref.unwrap().def_id).unwrap());
        if let NodeItem(trait_ref_item) = trait_ref_node {
            if let ItemTrait(.., ref items) = trait_ref_item.node {
                let tr = ty::TraitRef {
                    def_id: is_impl_trait_ref.unwrap().def_id,
                    substs: Substs::identity_for_item(tcx, is_impl_trait_ref.unwrap().def_id)
                };
                is_trait = Some((tr, items));
            }
        }
    }
    (generics, None)
}

