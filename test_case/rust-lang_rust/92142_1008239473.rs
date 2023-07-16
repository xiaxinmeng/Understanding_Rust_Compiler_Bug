diff

--- expected_show_coverage.unused_mod.txt	2022-01-05 19:57:06.401450600 +0000
+++ /d/a/rust/rust/build/x86_64-pc-windows-msvc/test/run-make-fulldeps/coverage-reports/coverage-reports/actual_show_coverage.unused_mod.txt	2022-01-05 20:53:32.785969200 +0000
@@ -1,8 +1,3 @@
-../coverage/lib/unused_mod_helper.rs:
-    1|      0|pub fn never_called_function() {
-    2|      0|    println!("I am never called");
-    3|      0|}
-
 ../coverage/unused_mod.rs:
     1|       |#[path = "lib/unused_mod_helper.rs"]
     2|       |mod unused_module;
@@ -11,3 +6,8 @@
     5|      1|    println!("hello world!");
     6|      1|}
 
+../coverage/lib/unused_mod_helper.rs:
+    1|      0|pub fn never_called_function() {
+    2|      0|    println!("I am never called");
+    3|      0|}
+
