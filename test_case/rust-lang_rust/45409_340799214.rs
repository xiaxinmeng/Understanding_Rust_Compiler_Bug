diff
diff --git a/src/librustc_typeck/check/_match.rs b/src/librustc_typeck/check/_match.rs
index ab8994bcae25..e25f7d796689 100644
--- a/src/librustc_typeck/check/_match.rs
+++ b/src/librustc_typeck/check/_match.rs
@@ -68,7 +69,7 @@ impl<'a, 'gcx, 'tcx> FnCtxt<'a, 'gcx, 'tcx> {
             PatKind::Binding(..) |
             PatKind::Ref(..) => false,
         };
-        if is_non_ref_pat && tcx.sess.features.borrow().match_default_bindings {
+        if is_non_ref_pat {
             debug!("pattern is non reference pattern");
             let mut exp_ty = self.resolve_type_vars_with_obligations(&expected);
