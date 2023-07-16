
diff --git a/compiler/rustc_lint/src/unused.rs b/compiler/rustc_lint/src/unused.rs
index 755e24d5413..a74a83567a3 100644
--- a/compiler/rustc_lint/src/unused.rs
+++ b/compiler/rustc_lint/src/unused.rs
@@ -512,7 +512,7 @@ fn emit_unused_delims_expr(
                 if value.span.from_expansion() || expr.span.from_expansion() {
                     (
                         value.span.with_hi(value.span.lo() + BytePos(1)),
-                        value.span.with_lo(value.span.hi() - BytePos(1)),
+                        value.span.with_lo(value.span.hi()),
                     )
                 } else {
                     (value.span.with_hi(expr.span.lo()), value.span.with_lo(expr.span.hi()))
