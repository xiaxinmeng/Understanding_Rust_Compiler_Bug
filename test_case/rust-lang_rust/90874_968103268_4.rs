
     #[unstable(feature = "range_offset", issue = "none")]
     pub fn offset_from(&self, position: Idx) -> RangeFrom<Idx> {
-        (self.start-position)..
+        (self.start - position)..
 }
 
Diff in /checkout/library/core/src/ops/range.rs at line 358:
     }
     }
 }
 
-impl<Idx: Add<Idx, Output=Idx> + Copy> RangeTo<Idx> {
+impl<Idx: Add<Idx, Output = Idx> + Copy> RangeTo<Idx> {
     /// Returns a new `RangeTo` analogous to this range, but moved to the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 373:
     /// 