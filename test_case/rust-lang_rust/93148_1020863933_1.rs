
-            hir::CaptureBy::Value if !place.deref_tys().any(ty::TyS::is_ref) => {
+            hir::CaptureBy::Value if !place.deref_tys().any(<Uniq<ty::TyS<'_>>>::is_ref) => {
