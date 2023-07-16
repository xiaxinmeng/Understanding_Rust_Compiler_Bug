diff
diff --git a/compiler/rustc_ast_lowering/src/path.rs b/compiler/rustc_ast_lowering/src/path.rs
index 6afed35..d43a034 100644
--- a/compiler/rustc_ast_lowering/src/path.rs
+++ b/compiler/rustc_ast_lowering/src/path.rs
@@ -322,7 +322,7 @@ impl<'a, 'hir> LoweringContext<'a, 'hir> {
                     AnonymousLifetimeMode::PassThrough | AnonymousLifetimeMode::ReportError => {
                         self.resolver.lint_buffer().buffer_lint_with_diagnostic(
                             ELIDED_LIFETIMES_IN_PATHS,
-                            CRATE_NODE_ID,
+                            segment.id,
                             path_span,
                             "hidden lifetime parameters in types are deprecated",
                             BuiltinLintDiagnostics::ElidedLifetimesInPaths(
