 diff
--- a/src/librustc_typeck/check/op.rs
+++ b/src/librustc_typeck/check/op.rs
@@ -18,6 +18,7 @@ use super::{
     method,
     FnCtxt,
 };
+use lint;
 use middle::def_id::DefId;
 use middle::ty::{Ty, HasTypeFlags, PreferMutLvalue};
 use syntax::ast;
@@ -71,6 +72,20 @@ pub fn check_binop<'a, 'tcx>(fcx: &FnCtxt<'a, 'tcx>,
     check_expr(fcx, lhs_expr);
     let lhs_ty = fcx.resolve_type_vars_if_possible(fcx.expr_ty(lhs_expr));

+    if let Some(&lhs_ty) = fcx.inh.tables.borrow().node_types.get(&lhs_expr.id) {
+        if fcx.infcx().type_var_diverges(lhs_ty) {
+            fcx.ccx.tcx.sess.add_lint(lint::builtin::UNREACHABLE_CODE, rhs_expr.id,
+                                      rhs_expr.span, "unreachable expression".to_string());
+        }
+    }
+
+    if let Some(&rhs_ty) = fcx.inh.tables.borrow().node_types.get(&rhs_expr.id) {
+        if fcx.infcx().type_var_diverges(rhs_ty) {
+            fcx.ccx.tcx.sess.add_lint(lint::builtin::UNREACHABLE_CODE, expr.id,
+                                      expr.span, "unreachable operation".to_string());
+        }
+    }
+
     match BinOpCategory::from(op) {
         BinOpCategory::Shortcircuit => {
             // && and || are a simple case.
