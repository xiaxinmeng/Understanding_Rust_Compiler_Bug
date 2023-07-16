diff
diff --git a/src/librustc_typeck/astconv.rs b/src/librustc_typeck/astconv.rs
index cde6eb22bb..21e2466092 100644
--- a/src/librustc_typeck/astconv.rs
+++ b/src/librustc_typeck/astconv.rs
@@ -1701,7 +1701,7 @@ impl<'o, 'gcx: 'tcx, 'tcx> dyn AstConv<'gcx, 'tcx> + 'o {
                 // `Self` in impl (we know the concrete type).
                 assert_eq!(opt_self_ty, None);
                 self.prohibit_generics(&path.segments);
-                tcx.at(span).type_of(def_id)
+                self.normalize_ty(span, tcx.at(span).type_of(def_id))
             }
             Def::SelfTy(Some(_), None) => {
                 // `Self` in trait.
