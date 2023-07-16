diff
-trait ExactSizeIterable<'a> : Iterable<'a> where Self::Iter : ExactSizeIterator {}
+trait ExactSizeIterable<'a> : Iterable<'a, Iter=<Self as ExactSizeIterable<'a>>::Iter> {
+    type Iter: ExactSizeIterator<'a>;
+}
