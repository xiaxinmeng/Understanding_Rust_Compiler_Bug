diff
diff --git a/src/libcore/ops/range.rs b/src/libcore/ops/range.rs
index d38b3516569..54672cd8bf6 100644
--- a/src/libcore/ops/range.rs
+++ b/src/libcore/ops/range.rs
@@ -349,32 +349,13 @@ pub struct RangeInclusive<Idx> {
     // accept non-PartialOrd types, also we want the constructor to be const.
 }
 
-trait RangeInclusiveEquality: Sized {
-    fn canonicalized_is_empty(range: &RangeInclusive<Self>) -> bool;
-}
-
-impl<T> RangeInclusiveEquality for T {
-    #[inline]
-    default fn canonicalized_is_empty(range: &RangeInclusive<Self>) -> bool {
-        range.is_empty.unwrap_or_default()
-    }
-}
-
-impl<T: PartialOrd> RangeInclusiveEquality for T {
-    #[inline]
-    fn canonicalized_is_empty(range: &RangeInclusive<Self>) -> bool {
-        range.is_empty()
-    }
-}
-
 #[stable(feature = "inclusive_range", since = "1.26.0")]
 impl<Idx: PartialEq> PartialEq for RangeInclusive<Idx> {
     #[inline]
     fn eq(&self, other: &Self) -> bool {
         self.start == other.start
             && self.end == other.end
-            && RangeInclusiveEquality::canonicalized_is_empty(self)
-                == RangeInclusiveEquality::canonicalized_is_empty(other)
+            && ((self.start == self.end) == (other.start == other.end))
     }
 }
 
@@ -386,7 +367,6 @@ impl<Idx: Hash> Hash for RangeInclusive<Idx> {
     fn hash<H: Hasher>(&self, state: &mut H) {
         self.start.hash(state);
         self.end.hash(state);
-        RangeInclusiveEquality::canonicalized_is_empty(self).hash(state);
     }
 }
 
