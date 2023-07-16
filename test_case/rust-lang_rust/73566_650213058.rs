diff
diff --git a/src/librustdoc/core.rs b/src/librustdoc/core.rs
index e18be549420..852f872205c 100644
--- a/src/librustdoc/core.rs
+++ b/src/librustdoc/core.rs
@@ -365,7 +365,10 @@ pub fn run_core(options: RustdocOptions) -> (clean::Crate, RenderInfo, RenderOpt
         crate_name,
         lint_caps,
         register_lints: None,
-        override_queries: None,
+        override_queries: Some(|_sess, local_providers, external_providers| {
+            local_providers.lint_mod = |_, _| {};
+            external_providers.lint_mod = |_, _| {};
+        }),
         registry: rustc_driver::diagnostics_registry(),
     };
