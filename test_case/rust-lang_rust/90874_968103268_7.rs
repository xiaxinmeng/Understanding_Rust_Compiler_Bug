
     #[unstable(feature = "range_offset", issue = "none")]
     pub fn offset_to(&self, position: Idx) -> RangeInclusive<Idx> {
-        Self {
-            start: position+self.start,
-            end: position+self.end,
-            exhausted: self.exhausted,
-        }
+        Self { start: position + self.start, end: position + self.end, exhausted: self.exhausted }
 }
 
Diff in /checkout/library/core/src/ops/range.rs at line 498:
Diff in /checkout/library/core/src/ops/range.rs at line 498:
-impl<Idx: Sub<Idx, Output=Idx> + Copy> RangeInclusive<Idx> {
+impl<Idx: Sub<Idx, Output = Idx> + Copy> RangeInclusive<Idx> {
     /// Returns a new `RangeInclusive` analogous to this range, but moved from the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 510:
     /// 