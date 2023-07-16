diff
    pub fn as_mut_ptr(&mut self) -> *mut T {
        // We shadow the slice method of the same name to avoid going through
        // `deref_mut`, which creates an intermediate reference.
-        let ptr = self.buf.ptr();
+        let ptr = self.buf.as_mut_ptr().cast::<T>();
        unsafe {
            assume(!ptr.is_null());
        }
