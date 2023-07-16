diff
--- a/src/librustc_save_analysis/dump_visitor.rs
+++ b/src/librustc_save_analysis/dump_visitor.rs
@@ -465,7 +465,7 @@ impl<'l, 'tcx: 'l, 'll, O: DumpOutput + 'll> DumpVisitor<'l, 'tcx, 'll, O> {
         // walk type and init value
         self.visit_ty(typ);
         if let Some(expr) = expr {
-            self.visit_expr(expr);
+            self.nest_tables(id, |v| v.visit_expr(expr));
         }
