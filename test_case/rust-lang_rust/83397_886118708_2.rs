diff
diff --git a/library/core/src/future/future.rs b/library/core/src/future/future.rs
index 15952c6806f..bffc5c430f7 100644
--- a/library/core/src/future/future.rs
+++ b/library/core/src/future/future.rs
@@ -1,5 +1,3 @@
-#![stable(feature = "futures_api", since = "1.36.0")]
-
 use crate::marker::Unpin;
 use crate::ops;
 use crate::pin::Pin;
diff --git a/library/core/src/hash/sip.rs b/library/core/src/hash/sip.rs
index 6178b0af137..ac7be9b390a 100644
--- a/library/core/src/hash/sip.rs
+++ b/library/core/src/hash/sip.rs
@@ -27,11 +27,6 @@ pub struct SipHasher13 {
 /// An implementation of SipHash 2-4.
 ///
 /// See: <https://131002.net/siphash/>
-#[unstable(feature = "hashmap_internals", issue = "none")]
-#[rustc_deprecated(
-    since = "1.13.0",
-    reason = "use `std::collections::hash_map::DefaultHasher` instead"
-)]
 #[derive(Debug, Clone, Default)]
 struct SipHasher24 {
     hasher: Hasher<Sip24Rounds>,
diff --git a/library/core/src/iter/adapters/mod.rs b/library/core/src/iter/adapters/mod.rs
index a3fbf4d9c38..8f315472085 100644
--- a/library/core/src/iter/adapters/mod.rs
+++ b/library/core/src/iter/adapters/mod.rs
@@ -196,7 +196,6 @@ fn ok<B, T>(mut f: impl FnMut(B, T) -> B) -> impl FnMut(B, T) -> Result<B, !> {
     }
 }
 
-#[unstable(issue = "none", feature = "inplace_iteration")]
 unsafe impl<S: Iterator, I, E> SourceIter for ResultShunt<'_, I, E>
 where
     I: SourceIter<Source = S>,
@@ -213,7 +212,6 @@ unsafe fn as_inner(&mut self) -> &mut S {
 // SAFETY: ResultShunt::next calls I::find, which has to advance `iter` in order to
 // return `Some(_)`. Since `iter` has type `I: InPlaceIterable` it's guaranteed that
 // at least one item will be moved out from the underlying source.
-#[unstable(issue = "none", feature = "inplace_iteration")]
 unsafe impl<I, T, E> InPlaceIterable for ResultShunt<'_, I, E> where
     I: Iterator<Item = Result<T, E>> + InPlaceIterable
 {
diff --git a/library/core/src/lib.rs b/library/core/src/lib.rs
index 540cdf124ee..f283f285e1f 100644
--- a/library/core/src/lib.rs
+++ b/library/core/src/lib.rs
@@ -313,7 +313,6 @@ pub mod assert_matches {
 // FIXME: This annotation should be moved into rust-lang/stdarch after clashing_extern_declarations is
 // merged. It currently cannot because bootstrap fails as the lint hasn't been defined yet.
 #[allow(clashing_extern_declarations)]
-#[unstable(feature = "stdsimd", issue = "48556")]
 mod core_arch;
 
 #[doc = include_str!("../../stdarch/crates/core_arch/src/core_arch_docs.md")]
diff --git a/library/core/src/ops/mod.rs b/library/core/src/ops/mod.rs
index 85e04740d96..f9fe121bbc5 100644
--- a/library/core/src/ops/mod.rs
+++ b/library/core/src/ops/mod.rs
@@ -187,7 +187,6 @@
 #[unstable(feature = "try_trait_v2", issue = "84277")]
 pub use self::try_trait::Try;
 
-#[unstable(feature = "try_trait_transition", reason = "for bootstrap", issue = "none")]
 pub(crate) use self::try_trait::Try as TryV2;
 
 #[unstable(feature = "generator_trait", issue = "43122")]
diff --git a/library/core/src/ptr/metadata.rs b/library/core/src/ptr/metadata.rs
index 287ae69acd1..bb7652a3526 100644
--- a/library/core/src/ptr/metadata.rs
+++ b/library/core/src/ptr/metadata.rs
@@ -1,5 +1,3 @@
-#![unstable(feature = "ptr_metadata", issue = "81513")]
-
 use crate::fmt;
 use crate::hash::{Hash, Hasher};
 
diff --git a/library/core/src/slice/iter.rs b/library/core/src/slice/iter.rs
index 5cbc6343e3a..5dfa1dc3441 100644
--- a/library/core/src/slice/iter.rs
+++ b/library/core/src/slice/iter.rs
@@ -880,7 +880,6 @@ fn next_back(&mut self) -> Option<&'a [T]> {
     }
 }
 
-#[stable(feature = "slice_rsplit", since = "1.27.0")]
 impl<'a, T, P> SplitIter for RSplit<'a, T, P>
 where
     P: FnMut(&T) -> bool,
@@ -936,7 +935,6 @@ fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
     }
 }
 
-#[stable(feature = "slice_rsplit", since = "1.27.0")]
 impl<'a, T, P> SplitIter for RSplitMut<'a, T, P>
 where
     P: FnMut(&T) -> bool,
diff --git a/library/core/src/slice/mod.rs b/library/core/src/slice/mod.rs
index de25c984abf..5087bf8ae89 100644
--- a/library/core/src/slice/mod.rs
+++ b/library/core/src/slice/mod.rs
@@ -1582,7 +1582,6 @@ pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
     ///     assert_eq!(right, []);
     /// }
     /// 