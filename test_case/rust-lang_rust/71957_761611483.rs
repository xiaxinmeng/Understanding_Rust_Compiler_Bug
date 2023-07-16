diff
diff --git a/compiler/rustc_ast_lowering/src/path.rs b/compiler/rustc_ast_lowering/src/path.rs
index 6afed35..82825c6 100644
--- a/compiler/rustc_ast_lowering/src/path.rs
+++ b/compiler/rustc_ast_lowering/src/path.rs
@@ -320,9 +320,10 @@ impl<'a, 'hir> LoweringContext<'a, 'hir> {
                         err.emit();
                     }
                     AnonymousLifetimeMode::PassThrough | AnonymousLifetimeMode::ReportError => {
+                        if path_span.ctxt().outer_expn_data().is_root() {
                             self.resolver.lint_buffer().buffer_lint_with_diagnostic(
                                 ELIDED_LIFETIMES_IN_PATHS,
-                            CRATE_NODE_ID,
+                                segment.id,
                                 path_span,
                                 "hidden lifetime parameters in types are deprecated",
                                 BuiltinLintDiagnostics::ElidedLifetimesInPaths(
@@ -337,6 +338,7 @@ impl<'a, 'hir> LoweringContext<'a, 'hir> {
                     }
                 }
             }
+        }
 
         let res = self.expect_full_res(segment.id);
