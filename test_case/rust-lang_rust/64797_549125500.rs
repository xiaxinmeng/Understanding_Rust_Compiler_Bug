diff
diff --git a/src/libsyntax/ast.rs b/src/libsyntax/ast.rs
index 8b967048848..9b6373bfb42 100644
--- a/src/libsyntax/ast.rs
+++ b/src/libsyntax/ast.rs
@@ -514,6 +514,10 @@ pub enum MetaItemKind {
     ///
     /// E.g., `feature = "foo"` as in `#[feature = "foo"]`.
     NameValue(Lit),
+    /// Value meta item.
+    ///
+    /// E.g., `path` as in `accessible(path)`.
+    Value(Lit),
 }

 /// A block (`{ .. }`).
