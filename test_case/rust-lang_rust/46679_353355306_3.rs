

--- src/main.rs
+++ src/main.rs
@@ -3,7 +3,7 @@
 #[derive(Copy, Clone, Debug)]
 #[repr(C)]
 pub struct MyObj {
-  pub a: [u8; 32],
+  pub a: [u8; 16],
 }

 extern { 
