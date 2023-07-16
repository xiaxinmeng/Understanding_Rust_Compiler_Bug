 diff
diff --git a/src/libsyntax/ext/expand.rs b/src/libsyntax/ext/expand.rs
index ee2cf90..fdb5716 100644
--- a/src/libsyntax/ext/expand.rs
+++ b/src/libsyntax/ext/expand.rs
@@ -80,7 +80,7 @@ pub fn expand_expr(e: P<ast::Expr>, fld: &mut MacroExpander) -> P<ast::Expr> {
             fully_expanded.map(|e| ast::Expr {
                 id: ast::DUMMY_NODE_ID,
                 node: e.node,
-                span: span,
+                span: fld.new_span(span),
             })
         }
