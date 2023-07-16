
 
 #![deny(missing_docs)]
Mismatch at tests/source/macro_not_expr.rs:1:
 macro_rules! test {
 macro_rules! test {
-    ($($t:tt)*) => {};
+    ($($t:tt)*) => {}
 
 fn main() {


Mismatch at tests/source/configs/format_macro_bodies/true.rs:1:
 // rustfmt-format_macro_bodies: true
 macro_rules! foo {
 macro_rules! foo {
-    ($a: ident : $b: ty) => {
-        $a(42): $b;
-    };
-    ($a: ident $b: ident $c: ident) => {
-        $a = $b + $c;
-    };
+    ($a: ident : $b: ty) => { $a(42): $b; };
+    ($a: ident $b: ident $c: ident) => { $a=$b+$c; };
 


Mismatch at tests/source/configs/format_macro_matchers/false.rs:1:
 // rustfmt-format_macro_matchers: false
 macro_rules! foo {
 macro_rules! foo {
-    ($a: ident : $b: ty) => {
-        $a(42): $b;
-    };
-    ($a: ident $b: ident $c: ident) => {
-        $a = $b + $c;
-    };
+    ($a: ident : $b: ty) => { $a(42): $b; };
+    ($a: ident $b: ident $c: ident) => { $a=$b+$c; };
 

Mismatch at tests/source/configs/format_macro_matchers/true.rs:1:
 // rustfmt-format_macro_matchers: true
 // rustfmt-format_macro_matchers: true
 
 macro_rules! foo {
-    ($a:ident : $b:ty) => {
-        $a(42): $b;
-    };
-    ($a:ident $b:ident $c:ident) => {
-        $a = $b + $c;
-    };
+    ($a: ident : $b: ty) => { $a(42): $b; };
+    ($a: ident $b: ident $c: ident) => { $a=$b+$c; };
 

Mismatch at tests/source/doc-comment-with-example.rs:6:
 /// 