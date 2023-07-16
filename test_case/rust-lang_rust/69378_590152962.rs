
$ git diff
diff --git a/src/librustc_typeck/check/expr.rs b/src/librustc_typeck/check/expr.rs
index 38d73256469..a3bb0c2fb81 100644
--- a/src/librustc_typeck/check/expr.rs
+++ b/src/librustc_typeck/check/expr.rs
@@ -1311,9 +1311,6 @@ impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
         kind_name: &str,
         ty_span: Span,
     ) {
-        if variant.recovered {
-            return;
-        }
         let mut err = self.type_error_struct_with_diag(
             field.ident.span,
             |actual| match ty.kind {

