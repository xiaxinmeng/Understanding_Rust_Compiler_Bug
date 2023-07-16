
     #[unstable(feature = "range_offset", issue = "none")]
     pub fn offset_to(&self, position: Idx) -> Range<Idx> {
-        (position+self.start)..(position+self.end)
+        (position + self.start)..(position + self.end)
 }
 
Diff in /checkout/library/core/src/ops/range.rs at line 120:
Diff in /checkout/library/core/src/ops/range.rs at line 120:
-impl<Idx: Sub<Idx, Output=Idx> + Copy> Range<Idx> {
+impl<Idx: Sub<Idx, Output = Idx> + Copy> Range<Idx> {
     /// Returns a new `Range` analogous to this range, but moved from the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 132:
     /// 