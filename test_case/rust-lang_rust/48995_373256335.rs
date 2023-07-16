diff
--- a/clippy_lints/src/utils/mod.rs
+++ b/clippy_lints/src/utils/mod.rs
@@ -306,11 +306,10 @@ pub fn implements_trait<'a, 'tcx>(
     ty_params: &[Ty<'tcx>],
 ) -> bool {
     let ty = cx.tcx.erase_regions(&ty);
-    let obligation =
-        cx.tcx
-            .predicate_for_trait_def(cx.param_env, traits::ObligationCause::dummy(), trait_id, 0, ty, ty_params);
+    let trait_ref = ty::TraitRef::new(trait_id,
+                                      cx.tcx.mk_substs_trait(ty, ty_params);
     cx.tcx.infer_ctxt().enter(|infcx| {
-        traits::SelectionContext::new(&infcx).evaluate_obligation_conservatively(&obligation)
+        infcx.predicate_must_hold(cx.param_env, trait_ref.to_predicate())
     })
 }
