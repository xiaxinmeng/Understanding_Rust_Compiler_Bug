diff
diff --git a/src/libcore/iter/mod.rs b/src/libcore/iter/mod.rs
index 2e08508..3a054f0 100644
--- a/src/libcore/iter/mod.rs
+++ b/src/libcore/iter/mod.rs
@@ -976,13 +976,13 @@ unsafe impl<A, B> TrustedLen for Zip<A, B>
 #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
 #[stable(feature = "rust1", since = "1.0.0")]
 #[derive(Clone)]
-pub struct Map<I, F> {
+pub struct Map<I, F: ?Sized> {
     iter: I,
     f: F,
 }
 
 #[stable(feature = "core_impl_debug", since = "1.9.0")]
-impl<I: fmt::Debug, F> fmt::Debug for Map<I, F> {
+impl<I: fmt::Debug, F: ?Sized> fmt::Debug for Map<I, F> {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         f.debug_struct("Map")
             .field("iter", &self.iter)
@@ -991,7 +991,7 @@ impl<I: fmt::Debug, F> fmt::Debug for Map<I, F> {
 }
 
 #[stable(feature = "rust1", since = "1.0.0")]
-impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
+impl<B, I: Iterator, F: ?Sized> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
     type Item = B;
 
     #[inline]
@@ -1003,7 +1003,10 @@ impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
     fn size_hint(&self) -> (usize, Option<usize>) {
         self.iter.size_hint()
     }
+}
 
+#[stable(feature = "rust1", since = "1.0.0")]
+impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
     fn fold<Acc, G>(self, init: Acc, mut g: G) -> Acc
         where G: FnMut(Acc, Self::Item) -> Acc,
     {
@@ -1013,7 +1016,7 @@ impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
 }
 
 #[stable(feature = "rust1", since = "1.0.0")]
-impl<B, I: DoubleEndedIterator, F> DoubleEndedIterator for Map<I, F> where
+impl<B, I: DoubleEndedIterator, F: ?Sized> DoubleEndedIterator for Map<I, F> where
     F: FnMut(I::Item) -> B,
 {
     #[inline]
@@ -1023,7 +1026,7 @@ impl<B, I: DoubleEndedIterator, F> DoubleEndedIterator for Map<I, F> where
 }
 
 #[stable(feature = "rust1", since = "1.0.0")]
-impl<B, I: ExactSizeIterator, F> ExactSizeIterator for Map<I, F>
+impl<B, I: ExactSizeIterator, F: ?Sized> ExactSizeIterator for Map<I, F>
     where F: FnMut(I::Item) -> B
 {
     fn len(&self) -> usize {
@@ -1036,16 +1039,16 @@ impl<B, I: ExactSizeIterator, F> ExactSizeIterator for Map<I, F>
 }
 
 #[unstable(feature = "fused", issue = "35602")]
-impl<B, I: FusedIterator, F> FusedIterator for Map<I, F>
+impl<B, I: FusedIterator, F: ?Sized> FusedIterator for Map<I, F>
     where F: FnMut(I::Item) -> B {}
 
 #[unstable(feature = "trusted_len", issue = "37572")]
-unsafe impl<B, I, F> TrustedLen for Map<I, F>
+unsafe impl<B, I, F: ?Sized> TrustedLen for Map<I, F>
     where I: TrustedLen,
           F: FnMut(I::Item) -> B {}
 
 #[doc(hidden)]
-unsafe impl<B, I, F> TrustedRandomAccess for Map<I, F>
+unsafe impl<B, I, F: ?Sized> TrustedRandomAccess for Map<I, F>
     where I: TrustedRandomAccess,
           F: FnMut(I::Item) -> B,
 {
