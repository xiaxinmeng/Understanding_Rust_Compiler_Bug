 diff
diff --git a/src/lib.rs b/src/lib.rs
index 3f39057..b5787de 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -23,7 +23,7 @@ fn log2_ceil(n: u8) -> u8 {
 fn lsb_differ_index(a: u8, b: u8) -> u8 {
     // xor => bits with differences are 1
     // trailing_zero => position of first difference
-    (a ^ b).trailing_zeros() as u8
+    ((a ^ b) as u16 | 0x100).trailing_zeros() as u8
 }

 #[test]
