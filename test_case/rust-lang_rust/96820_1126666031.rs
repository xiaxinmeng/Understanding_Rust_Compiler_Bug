
@@ -536,7 +549,7 @@ fn deref_mut(&mut self) -> &mut T {
 impl<T: ?Sized> Drop for RwLockReadGuard<'_, T> {
     fn drop(&mut self) {
         unsafe {
-            self.lock.inner.read_unlock();
+            self.inner_lock.read_unlock();
         }
     }
 }
 