diff
compiler/rustc_passes/src/region.rs
--- INDEX/compiler/rustc_passes/src/region.rs
+++ WORKDIR/compiler/rustc_passes/src/region.rs
@@ -676,6 +676,12 @@ fn record_rvalue_scope<'tcx>(
                 | hir::ExprKind::Unary(hir::UnOp::Deref, ref subexpr)
                 | hir::ExprKind::Field(ref subexpr, _)
                 | hir::ExprKind::Index(ref subexpr, _) => {
+                    // rust#72956: do not artificially extend `var` region in `let x = var[i];`.
+                    if let hir::ExprKind::Path(hir::QPath::Resolved(_, path)) = subexpr.kind {
+                        if let hir::def::Res::Local(_) = path.res {
+                            return;
+                        }
+                    }
                     expr = &subexpr;
                 }
                 _ => {
