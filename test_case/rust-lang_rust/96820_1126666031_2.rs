
@@ -468,12 +470,23 @@ fn from(t: T) -> Self {
 }

 impl<'rwlock, T: ?Sized> RwLockReadGuard<'rwlock, T> {
+    /// Create a new instance of `RwLockReadGuard<T>` from a `RwLock<T>`.
+    ///
+    /// It is safe to call this function if and only if `lock.inner.read()` (or
+    /// `lock.inner.try_read()`) has been successfully called before instantiating this object.
     unsafe fn new(lock: &'rwlock RwLock<T>) -> LockResult<RwLockReadGuard<'rwlock, T>> {
-        poison::map_result(lock.poison.borrow(), |_| RwLockReadGuard { lock })
+        poison::map_result(lock.poison.borrow(), |_| RwLockReadGuard {
+            data: NonNull::new_unchecked(lock.data.get()),
+            inner_lock: &lock.inner,
+        })
     }
 }
 