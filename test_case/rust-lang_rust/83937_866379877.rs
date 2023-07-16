diff
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -19,7 +19,8 @@ pub trait CountingSort {
 impl CountingSort for [u8] {
     #[inline] // when this is removed, everything works well
     fn sort(&mut self) {
-        let _counts = [0usize; u8::MAX.into_usize() + 1];
+        let _counts = [0usize; u8::MAX.into_usize() * 0];
+        &_counts;
     }
 }
