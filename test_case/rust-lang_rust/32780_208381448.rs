 diff
diff --git a/src/librustc_typeck/check/closure.rs b/src/librustc_typeck/check/closure.rs
index 84c277a..ed58070 100644
--- a/src/librustc_typeck/check/closure.rs
+++ b/src/librustc_typeck/check/closure.rs
@@ -47,7 +47,8 @@ fn check_closure<'a,'tcx>(fcx: &FnCtxt<'a,'tcx>,
                           expected_sig: Option<ty::FnSig<'tcx>>) {
     let expr_def_id = fcx.tcx().map.local_def_id(expr.id);

-    debug!("check_closure opt_kind={:?} expected_sig={:?}",
+    debug!("check_closure expr.id={:?} opt_kind={:?} expected_sig={:?}",
+           expr.id,
            opt_kind,
            expected_sig);

@@ -179,9 +180,16 @@ fn deduce_expectations_from_obligations<'a,'tcx>(
                 ty::Predicate::TypeOutlives(..) => None,
                 ty::Predicate::WellFormed(..) => None,
                 ty::Predicate::ObjectSafe(..) => None,
-                ty::Predicate::ClosureKind(_closure_def_id, kind) => {
-                    return Some(kind);
-                }
+
+                // NB: This predicate is created by breaking down a
+                // `ClosureType: FnFoo()` predicate, where
+                // `ClosureType` represents some `TyClosure`. It can't
+                // possibly be referring to the current closure,
+                // because we haven't produced the `TyClosure` for
+                // this closure yet; this is exactly why the other
+                // code is looking for a self type of a unresolved
+                // inference variable.
+                ty::Predicate::ClosureKind(..) => None,
             };
             opt_trait_ref
                 .and_then(|trait_ref| self_type_matches_expected_vid(fcx, trait_ref, expected_vid))
