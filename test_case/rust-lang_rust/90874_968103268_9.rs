
     #[unstable(feature = "range_offset", issue = "none")]
     pub fn offset_to(&self, position: Idx) -> RangeToInclusive<Idx> {
-        ..=position+self.end
+        ..=position + self.end
 }
 
Diff in /checkout/library/core/src/ops/range.rs at line 782:
Diff in /checkout/library/core/src/ops/range.rs at line 782:
-impl<Idx: Sub<Idx, Output=Idx> + Copy> RangeToInclusive<Idx> {
+impl<Idx: Sub<Idx, Output = Idx> + Copy> RangeToInclusive<Idx> {
     /// Returns a new `RangeToInclusive` analogous to this range, but moved from the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 794:
     /// 