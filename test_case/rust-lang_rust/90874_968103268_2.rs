
     #[unstable(feature = "range_offset", issue = "none")]
     pub fn offset_from(&self, position: Idx) -> Range<Idx> {
-        (self.start-position)..(self.end-position)
+        (self.start - position)..(self.end - position)
 }
 
Diff in /checkout/library/core/src/ops/range.rs at line 239:
     }
     }
 }
 
-impl<Idx: Add<Idx, Output=Idx> + Copy> RangeFrom<Idx> {
+impl<Idx: Add<Idx, Output = Idx> + Copy> RangeFrom<Idx> {
     /// Returns a new `RangeFrom` analogous to this range, but moved to the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 254:
     /// 