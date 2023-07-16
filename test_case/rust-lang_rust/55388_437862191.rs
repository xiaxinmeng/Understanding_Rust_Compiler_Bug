
let drop_type = location.ty(mir, tcx).to_ty(tcx);
// get the drop trait's DefId
let drop_trait = tcx.lang_items().drop_trait().unwrap();
// get DefId of its method
let drop_fn = tcx.associated_items(drop_trait).next().unwrap();
