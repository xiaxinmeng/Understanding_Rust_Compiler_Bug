 diff
--- a/foo_macros.rs
+++ b/foo_macros.rs
@@ -99,6 +99,6 @@ fn expand_foo(cx: &mut ExtCtxt, sp: Span, args: &[ast::TokenTree])
 #[plugin_registrar]
 pub fn plugin_registrar(reg: &mut Registry) {
     let expander: MacroExpanderFn = expand_foo;
-    reg.register_syntax_extension(token::intern("foo"),
-                                  NormalTT(Box::new(expander), None, true));
+    reg.syntax_exts.push((token::intern("foo"),
+                          NormalTT(Box::new(expander), None, true)));
 }
