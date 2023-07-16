diff
diff --git a/src/librustc/hir/lowering.rs b/src/librustc/hir/lowering.rs
index 196f787998..64333a8957 100644
--- a/src/librustc/hir/lowering.rs
+++ b/src/librustc/hir/lowering.rs
@@ -3123,7 +3123,7 @@ impl<'a> LoweringContext<'a> {
             ExprKind::Range(Some(ref e1), Some(ref e2), RangeLimits::Closed) => {
                 // FIXME: Use head_sp directly after RangeInclusive::new() is stabilized in stage0.
                 let span = self.allow_internal_unstable(CompilerDesugaringKind::DotFill, e.span);
-                let id = self.lower_node_id(e.id);
+                let id = self.next_id();
                 let e1 = self.lower_expr(e1);
                 let e2 = self.lower_expr(e2);
                 let ty_path = P(self.std_path(span, &["ops", "RangeInclusive"], false));
