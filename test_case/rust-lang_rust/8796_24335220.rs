
diff --git a/src/test/run-pass/lint-cstack.rs b/src/test/run-pass/lint-cstack.rs
index 8e5ab54..b9e61b4 100644
--- a/src/test/run-pass/lint-cstack.rs
+++ b/src/test/run-pass/lint-cstack.rs
@@ -8,8 +8,10 @@
 // option. This file may not be copied, modified, or distributed
 // except according to those terms.

+use std::libc;
+
 extern {
-    fn rust_get_test_int() -> std::libc::intptr_t;
+    fn rust_get_test_int() -> libc::intptr_t;
 }

 trait A {
