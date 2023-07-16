
 #[inline(always)]
 pub(super) fn make_bitmask(n: u8) -> u8 {
     debug_assert!(n < 8);
@@ -22,13 +18,6 @@
 ///
 /// # Example
 ///
-/// 