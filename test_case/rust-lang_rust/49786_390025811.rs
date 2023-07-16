diff
diff --git a/src/librustc_typeck/check/mod.rs b/src/librustc_typeck/check/mod.rs
index db859e4205..ef5599f82b 100644
--- a/src/librustc_typeck/check/mod.rs
+++ b/src/librustc_typeck/check/mod.rs
@@ -2381,9 +2381,8 @@ impl<'a, 'gcx, 'tcx> FnCtxt<'a, 'gcx, 'tcx> {
             // If some lookup succeeds, write callee into table and extract index/element
             // type from the method signature.
             // If some lookup succeeded, install method in table
-            let input_ty = self.next_ty_var(TypeVariableOrigin::AutoDeref(base_expr.span));
             let method = self.try_overloaded_place_op(
-                expr.span, self_ty, &[input_ty], needs, PlaceOp::Index);
+                expr.span, self_ty, &[index_ty], needs, PlaceOp::Index);

             let result = method.map(|ok| {
                 debug!("try_index_step: success, using overloaded indexing");
@@ -2418,7 +2417,7 @@ impl<'a, 'gcx, 'tcx> FnCtxt<'a, 'gcx, 'tcx> {
                 self.apply_adjustments(base_expr, adjustments);

                 self.write_method_call(expr.hir_id, method);
-                (input_ty, self.make_overloaded_place_return_type(method).ty)
+                (index_ty, self.make_overloaded_place_return_type(method).ty)
             });
             if result.is_some() {
                 return result;
