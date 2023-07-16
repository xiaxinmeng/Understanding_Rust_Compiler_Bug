diff
--- a/compiler/rustc_mir_build/src/thir/cx/mod.rs
+++ b/compiler/rustc_mir_build/src/thir/cx/mod.rs
@@ -67,12 +67,6 @@ fn new(tcx: TyCtxt<'tcx>, def: ty::WithOptConstParam<LocalDefId>) -> Cx<'tcx> {
 
         match self.tcx.at(sp).lit_to_const(LitToConstInput { lit, ty, neg }) {
             Ok(c) => c,
-            Err(LitToConstError::UnparseableFloat) => {
-                // FIXME(#31407) this is only necessary because float parsing is buggy
-                self.tcx.sess.span_err(sp, "could not evaluate float literal (see issue #31407)");
-                // create a dummy value and continue compiling
-                self.tcx.const_error(ty)
-            }
--- a/compiler/rustc_mir_build/src/thir/constant.rs
+++ b/compiler/rustc_mir_build/src/thir/constant.rs
@@ -47,7 +47,7 @@
             trunc(if neg { (*n as i128).overflowing_neg().0 as u128 } else { *n })?
         }
         (ast::LitKind::Float(n, _), ty::Float(fty)) => {
-            parse_float(*n, *fty, neg).map_err(|_| LitToConstError::UnparseableFloat)?
+            parse_float(*n, *fty, neg).map_err(|_| LitToConstError::Reported)?
         }
 --- a/compiler/rustc_mir_build/src/thir/pattern/mod.rs
+++ b/compiler/rustc_mir_build/src/thir/pattern/mod.rs
@@ -563,10 +562,6 @@ fn lower_lit(&mut self, expr: &'tcx hir::Expr<'tcx>) -> PatKind<'tcx> {
                 LitToConstInput { lit: &lit.node, ty: self.typeck_results.expr_ty(expr), neg };
             match self.tcx.at(expr.span).lit_to_const(lit_input) {
                 Ok(val) => *self.const_to_pat(val, expr.hir_id, lit.span, false).kind,
-                Err(LitToConstError::UnparseableFloat) => {
-                    self.errors.push(PatternError::FloatBug);
-                    PatKind::Wild
-                }
                 Err(LitToConstError::Reported) => PatKind::Wild,
                 Err(LitToConstError::TypeError) => bug!("lower_lit: had type error"),
             }
