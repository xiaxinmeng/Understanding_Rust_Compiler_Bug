diff
diff --git a/compiler/rustc_parse/src/parser/expr.rs b/compiler/rustc_parse/src/parser/expr.rs
index 8fb84f81808..afdd196491c 100644
--- a/compiler/rustc_parse/src/parser/expr.rs
+++ b/compiler/rustc_parse/src/parser/expr.rs
@@ -1645,7 +1645,11 @@ pub(super) fn recover_unclosed_char(
                 .emit();
         }
         ast::Lit {
-            token_lit: token::Lit::new(token::LitKind::Char, lifetime.name, None),
+            token_lit: token::Lit::new(
+                token::LitKind::Char,
+                lifetime.without_first_quote().name,
+                None,
+            ),
             kind: ast::LitKind::Char(lifetime.name.as_str().chars().next().unwrap_or('_')),
             span: lifetime.span,
         }
