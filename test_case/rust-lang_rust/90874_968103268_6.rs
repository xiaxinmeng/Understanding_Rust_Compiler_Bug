
     #[unstable(feature = "range_offset", issue = "none")]
     pub fn offset_from(&self, position: Idx) -> RangeTo<Idx> {
-        ..(self.end-position)
+        ..(self.end - position)
 }
 
Diff in /checkout/library/core/src/ops/range.rs at line 472:
     pub(crate) exhausted: bool,
     pub(crate) exhausted: bool,
 }
 
-impl<Idx: Add<Idx, Output=Idx> + Copy> RangeInclusive<Idx> {
+impl<Idx: Add<Idx, Output = Idx> + Copy> RangeInclusive<Idx> {
     /// Returns a new `RangeInclusive` analogous to this range, but moved to the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 487:
     /// 