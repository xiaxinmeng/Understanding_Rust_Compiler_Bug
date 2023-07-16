
 
 #![deny(missing_docs)]
Mismatch at tests/source/macro_not_expr.rs:1:
 macro_rules! test {
 macro_rules! test {
-    ($($t:tt)*) => {};
+    ($($t:tt)*) => {}
 
 fn main() {

Mismatch at tests/source/macros.rs:146:
Mismatch at tests/source/macros.rs:146:
 
 fn issue1178() {
     macro_rules! foo {
-        (#[$attr:meta] $name:ident) => {};
+        (#[$attr:meta] $name:ident) => {}
 
     foo!(

Mismatch at tests/source/macros.rs:264:
Mismatch at tests/source/macros.rs:264:
 
 // #878
 macro_rules! try_opt {
-    ($expr:expr) => {
-        match $expr {
-            Some(val) => val,
+    ($expr:expr) => (match $expr {
+        Some(val) => val,
-            None => {
-                return None;
-            }
-        }
---
 
-macro foo() {}
+macro foo() {
 
-pub macro bar($x:ident + $y:expr;) {
+}
+
+
+pub macro bar($x:ident+$y:expr; ) {
     fn foo($x: Foo) {
-        long_function(
-            a_long_argument_to_a_long_function_is_what_this_is(AAAAAAAAAAAAAAAAAAAAAAAAAAAA),
-            $x.bar($y),
-        );
+    long_function(a_long_argument_to_a_long_function_is_what_this_is(AAAAAAAAAAAAAAAAAAAAAAAAAAAA),
+                  $x.bar($y));
 }
 

Mismatch at tests/source/macros.rs:921:
---
-    () => {{}};
+    () => {{}}
 }
 
 macro lex_err($kind: ident $(, $body: expr)*) {
Mismatch at tests/source/issue-3055/original.rs:32:
 ///
 ///
 /// Should format with rust attribute
 /// 