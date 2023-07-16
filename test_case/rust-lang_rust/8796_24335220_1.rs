
diff --git a/src/test/run-pass/lint-cstack.rs b/src/test/run-pass/lint-cstack.rs
index 8e5ab54..c665342 100644
--- a/src/test/run-pass/lint-cstack.rs
+++ b/src/test/run-pass/lint-cstack.rs
@@ -9,7 +9,7 @@
 // except according to those terms.

 extern {
-    fn rust_get_test_int() -> std::libc::intptr_t;
+    fn rust_get_test_int() -> ::std::libc::intptr_t;
 }

 trait A {
