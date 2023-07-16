rust
let impl1_trait_ref = tcx.impl_trait_ref(impl1_def_id).unwrap().subst(tcx, ty::InternalSubsts::identity_for_item(tcx, impl1_def_id));
