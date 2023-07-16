
@@ -119,10 +121,13 @@ pub fn translate_substs<'a, 'gcx, 'tcx>(infcx: &InferCtxt<'a, 'gcx, 'tcx>,
 /// whichever applies.
 pub fn find_associated_item<'a, 'tcx>(
     tcx: TyCtxt<'a, 'tcx, 'tcx>,
+    param_env: ty::ParamEnv<'tcx>,
     item: &ty::AssociatedItem,
     substs: &'tcx Substs<'tcx>,
     impl_data: &super::VtableImplData<'tcx, ()>,
 ) -> (DefId, &'tcx Substs<'tcx>) {
+    debug!("find_associated_item({:?}, {:?}, {:?}, {:?})",
+           param_env, item, substs, impl_data);
     assert!(!substs.needs_infer());
 
     let trait_def_id = tcx.trait_id_of_impl(impl_data.impl_def_id).unwrap();
@@ -132,7 +137,7 @@ pub fn find_associated_item<'a, 'tcx>(
     match ancestors.defs(tcx, item.ident, item.kind, trait_def_id).next() {
         Some(node_item) => {
             let substs = tcx.infer_ctxt().enter(|infcx| {
-                let param_env = ty::ParamEnv::reveal_all();
+                let param_env = param_env.with_reveal_all();
                 let substs = substs.rebase_onto(tcx, trait_def_id, impl_data.substs);
                 let substs = translate_substs(&infcx, param_env, impl_data.impl_def_id,
                                               substs, node_item.node);
