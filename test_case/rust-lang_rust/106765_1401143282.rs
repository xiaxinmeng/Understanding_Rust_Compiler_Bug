diff
diff --git a/compiler/rustc_ast_lowering/src/expr.rs b/compiler/rustc_ast_lowering/src/expr.rs
index c3611b2f522..4b0e70e6a12 100644
--- a/compiler/rustc_ast_lowering/src/expr.rs
+++ b/compiler/rustc_ast_lowering/src/expr.rs
@@ -685,6 +685,12 @@ pub(super) fn make_async_expr(
             );
         }

+        if let Some(attrs) = self.attrs.get(&outer_hir_id.local_id)
+            && let Some(found) = attrs.into_iter().find(|attr| attr.has_name(sym::inline)) {
+                // Should probably copy only certain fields from the attribute
+                self.lower_attrs(hir_id, &[found.clone()]);
+            }
+
         let generator = hir::Expr { hir_id, kind: generator_kind, span: self.lower_span(span) };

         // FIXME(swatinem):
