
$ git diff
diff --git a/src/librustc_typeck/check/expr.rs b/src/librustc_typeck/check/expr.rs
index 38d73256469..bd80cc31e3c 100644
--- a/src/librustc_typeck/check/expr.rs
+++ b/src/librustc_typeck/check/expr.rs
@@ -1218,8 +1218,8 @@ impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
 
                 self.field_ty(field.span, v_field, substs)
             } else {
-                error_happened = true;
                 if let Some(prev_span) = seen_fields.get(&ident) {
+                    error_happened = true;
                     let mut err = struct_span_err!(
                         self.tcx.sess,
                         field.ident.span,
@@ -1232,7 +1232,8 @@ impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
                     err.span_label(*prev_span, format!("first use of `{}`", ident));
 
                     err.emit();
-                } else {
+                } else if !variant.recovered {
+                    error_happened = true;
                     self.report_unknown_field(adt_ty, variant, field, ast_fields, kind_name, span);
                 }
 
@@ -1311,9 +1312,6 @@ impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
         kind_name: &str,
         ty_span: Span,
     ) {
-        if variant.recovered {
-            return;
-        }
         let mut err = self.type_error_struct_with_diag(
             field.ident.span,
             |actual| match ty.kind {
