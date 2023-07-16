
     #[unstable(feature = "range_offset", issue = "none")]
     pub fn offset_to(&self, position: Idx) -> RangeFrom<Idx> {
-        (position+self.start)..
+        (position + self.start)..
 }
 
Diff in /checkout/library/core/src/ops/range.rs at line 261:
Diff in /checkout/library/core/src/ops/range.rs at line 261:
-impl<Idx: Sub<Idx, Output=Idx> + Copy> RangeFrom<Idx> {
+impl<Idx: Sub<Idx, Output = Idx> + Copy> RangeFrom<Idx> {
     /// Returns a new `RangeFrom` analogous to this range, but moved from the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 273:
     /// 