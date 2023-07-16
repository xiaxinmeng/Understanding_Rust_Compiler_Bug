
diff --git a/src/libstd/cell.rs b/src/libstd/cell.rs
index 6fb4ce4..582e989 100644
--- a/src/libstd/cell.rs
+++ b/src/libstd/cell.rs
@@ -12,6 +12,7 @@

 use prelude::*;
 use cast;
+#[cfg(stage0)]
 use unstable::intrinsics;
 use util::NonCopyable;

@@ -65,18 +66,15 @@ impl<T: ::kinds::Pod> Cell<T> {
     /// Returns a copy of the contained value.
     #[inline]
     pub fn get(&self) -> T {
-        unsafe {
-            let mut result = intrinsics::uninit();
-            intrinsics::copy_nonoverlapping_memory(&mut result, &self.value, 1);
-            result
-        }
+        self.value // because this is POD, we can copy it
     }

     /// Sets the contained value.
     #[inline]
     pub fn set(&self, value: T) {
         unsafe {
-            intrinsics::copy_nonoverlapping_memory(cast::transmute_mut(&self.value), &value, 1)
+            let p: &mut T = cast::transmute_mut(&self.value);
+            *p = value;
         }
     }
 }
