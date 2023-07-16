
 @@ -512,7 +525,7 @@ impl<T: ?Sized> Deref for RwLockReadGuard<'_, T> {
     type Target = T;

     fn deref(&self) -> &T {
-        unsafe { &*self.lock.data.get() }
+        unsafe { self.data.as_ref() }
     }
 }
 