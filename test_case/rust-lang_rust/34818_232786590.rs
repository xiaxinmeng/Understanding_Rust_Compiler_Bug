 diff
diff --git a/src/libcollections/vec.rs b/src/libcollections/vec.rs
index f6861c0..b51b971 100644
--- a/src/libcollections/vec.rs
+++ b/src/libcollections/vec.rs
@@ -1604,8 +1604,8 @@ impl<'a, T> FromIterator<T> for Cow<'a, [T]> where T: Clone {

 /// An iterator that moves out of a vector.
 ///
-/// This `struct` is created by the `into_iter` method on [`Vec`] (provided by
-/// the [`IntoIterator`] trait).
+/// This `struct` is created by the `into_iter` method on [`Vec`][`Vec`] (provided
+/// by the [`IntoIterator`] trait).
 ///
 /// [`Vec`]: struct.Vec.html
 /// [`IntoIterator`]: ../iter/trait.IntoIterator.html
