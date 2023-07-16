 diff
diff --git a/src/librustrt/lib.rs b/src/librustrt/lib.rs
index 02ca7d3..e4935a5 100644
--- a/src/librustrt/lib.rs
+++ b/src/librustrt/lib.rs
@@ -27,6 +27,7 @@
 extern crate alloc;
 extern crate libc;
 extern crate collections;
+extern crate unicode;

 #[cfg(test)] extern crate "rustrt" as realrustrt;
 #[cfg(test)] extern crate test;
@@ -81,6 +82,7 @@ pub fn init(argc: int, argv: *const *const u8) {
     collections::fixme_14344_be_sure_to_link_to_collections();
     alloc::fixme_14344_be_sure_to_link_to_collections();
     libc::issue_14344_workaround();
+    unicode::issue_14344_workaround();
 }

 /// Enqueues a procedure to run when the runtime is cleaned up
diff --git a/src/libunicode/lib.rs b/src/libunicode/lib.rs
index 1f75daa..b83a2cc 100644
--- a/src/libunicode/lib.rs
+++ b/src/libunicode/lib.rs
@@ -82,3 +82,6 @@ mod std {
     pub use core::clone;
     pub use core::cmp;
 }
+
+#[doc(hidden)]
+pub fn issue_14344_workaround() {}
