
     #[unstable(feature = "range_offset", issue = "none")]
     pub fn offset_from(&self, position: Idx) -> RangeInclusive<Idx> {
-            start: self.start-position,
-            start: self.start-position,
-            end: self.end-position,
-            exhausted: self.exhausted,
-        }
+        Self { start: self.start - position, end: self.end - position, exhausted: self.exhausted }
 }
 
Diff in /checkout/library/core/src/ops/range.rs at line 760:
     }
     }
 }
 
-impl<Idx: Add<Idx, Output=Idx> + Copy> RangeToInclusive<Idx> {
+impl<Idx: Add<Idx, Output = Idx> + Copy> RangeToInclusive<Idx> {
     /// Returns a new `RangeToInclusive` analogous to this range, but moved to the
     /// given position.
Diff in /checkout/library/core/src/ops/range.rs at line 775:
     /// 