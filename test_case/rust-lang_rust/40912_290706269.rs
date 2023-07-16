diff
diff --git a/src/impl_views.rs b/src/impl_views.rs
index c26a4ef1..ac2f787c 100644
--- a/src/impl_views.rs
+++ b/src/impl_views.rs
@@ -19,6 +19,7 @@ use StrideShape;
 /// Methods for read-only array views `ArrayView<'a, A, D>`
 ///
 /// Note that array views implement traits like [`From`][f] and `IntoIterator` too.
+///
 /// [f]: #method.from
 impl<'a, A, D> ArrayBase<ViewRepr<&'a A>, D>
     where D: Dimension,
