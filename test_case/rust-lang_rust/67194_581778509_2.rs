diff,rust
-impl<T: PartialOrd> RangeInclusiveEquality for T {
+impl<T: Step> RangeInclusiveEquality for T {
     #[inline]
     fn canonicalized_is_empty(range: &RangeInclusive<Self>) -> bool {
         range.is_empty()
     }
 }
