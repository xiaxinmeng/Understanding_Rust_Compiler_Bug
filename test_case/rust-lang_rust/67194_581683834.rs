diff
diff --git a/src/libcore/iter/range.rs b/src/libcore/iter/range.rs
index eac3c107d22..640ea417207 100644
--- a/src/libcore/iter/range.rs
+++ b/src/libcore/iter/range.rs
@@ -341,10 +341,10 @@ impl<A: Step> Iterator for ops::RangeInclusive<A> {
 
     #[inline]
     fn next(&mut self) -> Option<A> {
-        self.compute_is_empty();
-        if self.is_empty.unwrap_or_default() {
+        if self.is_empty() {
             return None;
         }
+        self.compute_is_empty();
         let is_iterating = self.start < self.end;
         self.is_empty = Some(!is_iterating);
         Some(if is_iterating {
@@ -369,10 +369,10 @@ impl<A: Step> Iterator for ops::RangeInclusive<A> {
 
     #[inline]
     fn nth(&mut self, n: usize) -> Option<A> {
-        self.compute_is_empty();
-        if self.is_empty.unwrap_or_default() {
+        if self.is_empty() {
             return None;
         }
+        self.compute_is_empty();
 
         if let Some(plus_n) = self.start.add_usize(n) {
             use crate::cmp::Ordering::*;
@@ -402,11 +402,10 @@ impl<A: Step> Iterator for ops::RangeInclusive<A> {
         F: FnMut(B, Self::Item) -> R,
         R: Try<Ok = B>,
     {
-        self.compute_is_empty();
-
         if self.is_empty() {
             return Try::from_ok(init);
         }
+        self.compute_is_empty();
 
         let mut accum = init;
 
@@ -445,10 +444,10 @@ impl<A: Step> Iterator for ops::RangeInclusive<A> {
 impl<A: Step> DoubleEndedIterator for ops::RangeInclusive<A> {
     #[inline]
     fn next_back(&mut self) -> Option<A> {
-        self.compute_is_empty();
-        if self.is_empty.unwrap_or_default() {
+        if self.is_empty() {
             return None;
         }
+        self.compute_is_empty();
         let is_iterating = self.start < self.end;
         self.is_empty = Some(!is_iterating);
         Some(if is_iterating {
@@ -461,10 +460,10 @@ impl<A: Step> DoubleEndedIterator for ops::RangeInclusive<A> {
 
     #[inline]
     fn nth_back(&mut self, n: usize) -> Option<A> {
-        self.compute_is_empty();
-        if self.is_empty.unwrap_or_default() {
+        if self.is_empty() {
             return None;
         }
+        self.compute_is_empty();
 
         if let Some(minus_n) = self.end.sub_usize(n) {
             use crate::cmp::Ordering::*;
@@ -494,11 +493,10 @@ impl<A: Step> DoubleEndedIterator for ops::RangeInclusive<A> {
         F: FnMut(B, Self::Item) -> R,
         R: Try<Ok = B>,
     {
-        self.compute_is_empty();
-
         if self.is_empty() {
             return Try::from_ok(init);
         }
+        self.compute_is_empty();
 
         let mut accum = init;
 
diff --git a/src/libcore/ops/range.rs b/src/libcore/ops/range.rs
index d38b3516569..72d66698520 100644
--- a/src/libcore/ops/range.rs
+++ b/src/libcore/ops/range.rs
@@ -349,32 +349,11 @@ pub struct RangeInclusive<Idx> {
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
-        self.start == other.start
-            && self.end == other.end
-            && RangeInclusiveEquality::canonicalized_is_empty(self)
-                == RangeInclusiveEquality::canonicalized_is_empty(other)
+        self.start == other.start && self.end == other.end && self.is_empty.unwrap_or_default()
     }
 }
 
@@ -386,7 +365,7 @@ impl<Idx: Hash> Hash for RangeInclusive<Idx> {
     fn hash<H: Hasher>(&self, state: &mut H) {
         self.start.hash(state);
         self.end.hash(state);
-        RangeInclusiveEquality::canonicalized_is_empty(self).hash(state);
+        self.is_empty.unwrap_or_default().hash(state);
     }
 }
 

