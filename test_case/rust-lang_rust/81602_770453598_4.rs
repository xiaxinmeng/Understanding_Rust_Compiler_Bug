
 #[inline(always)]
 pub(super) fn mask_bits(src: u8, dst: u8, mask: u8) -> u8 {
     (src & mask) | (dst & !mask)
@@ -38,10 +27,6 @@
 ///
 /// # Example
 ///
-/// 