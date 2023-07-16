
diff --git a/src/librustc_typeck/check/expr.rs b/src/librustc_typeck/check/expr.rs
index 38d73256469..14615521749 100644
--- a/src/librustc_typeck/check/expr.rs
+++ b/src/librustc_typeck/check/expr.rs
@@ -1312,6 +1312,7 @@ impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
         ty_span: Span,
     ) {
         if variant.recovered {
+            self.set_tainted_by_errors();
             return;
         }
         let mut err = self.type_error_struct_with_diag(
