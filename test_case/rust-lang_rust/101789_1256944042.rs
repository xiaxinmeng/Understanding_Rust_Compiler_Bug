
compiler/rustc_parse/src/parser/item.rs at line 1735:
                 err.help("unlike in C++, Java, and C#, functions are declared in `impl` blocks");
                 err.help("see https://doc.rust-lang.org/book/ch05-03-method-syntax.html for more information");
                 err
-            }
-
-            else {
+            } else {
                 let mut err = self.expected_ident_found();
                 if self.check_ident() {
                     self.bump();
