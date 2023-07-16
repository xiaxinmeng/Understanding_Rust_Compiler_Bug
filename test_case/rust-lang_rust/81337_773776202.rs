diff
diff --git a/compiler/rustc_parse/src/parser/mod.rs b/compiler/rustc_parse/src/parser/mod.rs
index 760e141b1be..501b6d61e61 100644
--- a/compiler/rustc_parse/src/parser/mod.rs
+++ b/compiler/rustc_parse/src/parser/mod.rs
@@ -974,7 +974,8 @@ fn parse_mac_args_common(&mut self, delimited_only: bool) -> PResult<'a, MacArgs
                     }

                     // Collect tokens because they are used during lowering to HIR.
-                    let expr = self.collect_tokens(|this| this.parse_expr())?;
+                    let (mut expr, tokens) = self.collect_tokens(|this| this.parse_expr())?;
+                    expr.tokens = tokens;
                     let span = expr.span;

                     match &expr.kind {
