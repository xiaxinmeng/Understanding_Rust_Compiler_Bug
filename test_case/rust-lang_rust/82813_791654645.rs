diff
diff --git a/compiler/rustc_lint/src/lib.rs b/compiler/rustc_lint/src/lib.rs
index 3f5da7910aa..94ca0df818b 100644
--- a/compiler/rustc_lint/src/lib.rs
+++ b/compiler/rustc_lint/src/lib.rs
@@ -343,8 +343,7 @@ fn register_builtins(store: &mut LintStore, no_interleave_lints: bool) {
         "intra_doc_link_resolution_failure",
         "use `rustdoc::broken_intra_doc_links` instead",
     );
-    // This doesn't actually do anything, it's just so `deny(rustdoc)` doesn't give warnings.
-    store.register_group(false, "rustdoc", None, vec![]);
+    store.register_removed("rustdoc", "use `rustdoc::all` instead");
 
     store.register_removed("unknown_features", "replaced by an error");
     store.register_removed("unsigned_negation", "replaced by negate_unsigned feature gate");
diff --git a/src/librustdoc/lint.rs b/src/librustdoc/lint.rs
index e8806c1b6d7..f0916c7aea2 100644
--- a/src/librustdoc/lint.rs
+++ b/src/librustdoc/lint.rs
@@ -175,7 +175,7 @@ pub(crate) fn init_lints<F>(
     lint_store.register_lints(&**RUSTDOC_LINTS);
     lint_store.register_group(
         true,
-        "rustdoc",
+        "rustdoc::all",
         None,
         RUSTDOC_LINTS.iter().map(|&lint| LintId::of(lint)).collect(),
     );
