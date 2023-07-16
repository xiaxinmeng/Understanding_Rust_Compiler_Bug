diff
@@ -802,7 +802,14 @@ impl LintPass for UnusedDocComment {
 }
 
 impl UnusedDocComment {
-    fn warn_if_doc(&self, cx: &EarlyContext, attrs: &[ast::Attribute]) {
+    fn warn_if_doc(
+        &self,
+        cx: &EarlyContext<'_>,
+        node_span: Span,
+        node_kind: &str,
+        is_macro_expansion: bool,
+        attrs: &[ast::Attribute]
+    ) {
         let mut attrs = attrs.into_iter().peekable();
 
         // Accumulate a single span for sugared doc comments.
@@ -825,27 +832,51 @@ impl UnusedDocComment {
             let span = sugared_span.take().unwrap_or_else(|| attr.span);
 
             if attr.name() == "doc" {
-                cx.struct_span_lint(
-                    UNUSED_DOC_COMMENTS,
-                    span,
-                    "doc comment not used by rustdoc",
-                ).emit();
+                let mut err = cx.struct_span_lint(UNUSED_DOC_COMMENTS, span, "unused doc comment");
+
+                err.span_label(
+                    node_span,
+                    format!("rustdoc does not generate documentation for {}", node_kind)
+                );
+
+                if is_macro_expansion {
+                    err.help("to document an item produced by a macro, \
+                              the macro must produce the documentation as part of its expansion");
+                }
+
+                err.emit();
             }
         }
     }
 }
