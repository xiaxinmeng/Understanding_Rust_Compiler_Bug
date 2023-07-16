
-    #[unstable(feature = "vec_into_iter_as_slice", issue = "0")]
+    #[unstable(feature = "vec_into_iter_as_slice", issue = "35601")]
     pub fn as_slice(&self) -> &[T] {
         unsafe {
             slice::from_raw_parts(self.ptr, self.len())
@@ -1747,7 +1747,7 @@ impl<T> IntoIter<T> {
     /// assert_eq!(into_iter.next().unwrap(), 'b');
     /// assert_eq!(into_iter.next().unwrap(), 'z');
     /// 