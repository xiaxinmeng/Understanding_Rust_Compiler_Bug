
     #[unstable(feature = "range_offset", issue = "none")]
     pub fn offset_to(&self, position: Idx) -> RangeTo<Idx> {
-        ..(position+self.end)
+        ..(position + self.end)
 }
 
Diff in /checkout/library/core/src/ops/range.rs at line 380:
Diff in /checkout/library/core/src/ops/range.rs at line 380:
-impl<Idx: Sub<Idx, Output=Idx> + Copy> RangeTo<Idx> {
+impl<Idx: Sub<Idx, Output = Idx> + Copy> RangeTo<Idx> {
     /// Returns a new `RangeTo` analogous to this range, but moved from the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 392:
     /// 