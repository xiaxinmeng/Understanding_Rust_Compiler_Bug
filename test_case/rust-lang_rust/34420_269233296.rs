diff
diff --git a/src/librustc_typeck/check/op.rs b/src/librustc_typeck/check/op.rs
index 442fdb6..ca809a3 100644
--- a/src/librustc_typeck/check/op.rs
+++ b/src/librustc_typeck/check/op.rs
@@ -188,7 +188,8 @@ impl<'a, 'gcx, 'tcx> FnCtxt<'a, 'gcx, 'tcx> {
                             lhs_ty);
 
                         if let TypeVariants::TyRef(_, ref ty_mut) = lhs_ty.sty {
-                            if self.lookup_op_method(expr, ty_mut.ty, vec![rhs_ty_var],
+                            if !self.infcx.type_moves_by_default(ty_mut.ty, lhs_expr.span) &&
+                                self.lookup_op_method(expr, ty_mut.ty, vec![rhs_ty_var],
                                     token::intern(name), trait_def_id,
                                     lhs_expr).is_ok() {
                                 err.span_note(
