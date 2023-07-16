 diff
diff --git a/src/libregex/test/mod.rs b/src/libregex/test/mod.rs
index a4c3a83..b315396 100644
--- a/src/libregex/test/mod.rs
+++ b/src/libregex/test/mod.rs
@@ -9,6 +9,7 @@
 // except according to those terms.

 #[cfg(not(stage1))]
+#[cfg(stage2)]
 #[phase(syntax)]
 extern crate regex_macros;

