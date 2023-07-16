diff
diff --git a/src/rustc/rustc.rs b/src/rustc/rustc.rs
index bfd01146d2..1c3d8b1da2 100644
--- a/src/rustc/rustc.rs
+++ b/src/rustc/rustc.rs
@@ -9,7 +9,9 @@
 // except according to those terms.

 #![feature(rustc_private)]
+#![feature(alloc_system)]

+extern crate alloc_system;
 extern crate rustc_driver;

 fn main() { rustc_driver::main() }
