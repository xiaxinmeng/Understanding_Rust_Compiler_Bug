diff
--- src/parser.rs.orig  2019-05-19 18:26:44.678343192 +0000
+++ src/parser.rs       2019-05-19 18:26:55.530510339 +0000
@@ -64,9 +64,6 @@
                 if used.get() {
                     continue
                 }
-                if !cfg!(feature = "strict-macro") {
-                    continue
-                }
                 let span = match attr {
                     $(BindgenAttr::$variant(span, ..) => span,)*
                 };
