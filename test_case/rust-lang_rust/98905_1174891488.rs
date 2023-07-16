diff
 impl<T, U> AsRef<U> for Cow<'_, T>
 where
-    T: AsRef<U>,
+    T: ?Sized + ToOwned + AsRef<U>,
     U: ?Sized,
 {
     fn as_ref(&self) -> &U {
         self.deref().as_ref()
     }
 }
