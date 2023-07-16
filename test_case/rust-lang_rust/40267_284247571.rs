
diff --git a/src/libcollections/vec.rs b/src/libcollections/vec.rs
index 3134e3c2ce..1e7c4feed3 100644
--- a/src/libcollections/vec.rs
+++ b/src/libcollections/vec.rs
@@ -1206,17 +1206,15 @@ impl<T: Clone> Vec<T> {
         let len = self.len();
 
         if new_len > len {
-            self.extend_with_element(new_len - len, value);
+            self.reserve(new_len - len);
+            unsafe { self.extend_with_element(new_len - len, value) };
         } else {
             self.truncate(new_len);
         }
     }
 
     /// Extend the vector by `n` additional clones of `value`.
-    fn extend_with_element(&mut self, n: usize, value: T) {
-        self.reserve(n);
-
-        unsafe {
+    unsafe fn extend_with_element(&mut self, n: usize, value: T) {
             let mut ptr = self.as_mut_ptr().offset(self.len() as isize);
             // Use SetLenOnDrop to work around bug where compiler
             // may not realize the store through `ptr` through self.set_len()
@@ -1238,7 +1236,6 @@ impl<T: Clone> Vec<T> {
             }
 
             // len set by scope guard
-        }
     }
 
     /// Clones and appends all elements in a slice to the `Vec`.
@@ -1345,7 +1342,11 @@ impl<T: PartialEq> Vec<T> {
 #[stable(feature = "rust1", since = "1.0.0")]
 pub fn from_elem<T: Clone>(elem: T, n: usize) -> Vec<T> {
     let mut v = Vec::with_capacity(n);
-    v.extend_with_element(n, elem);
+    unsafe {
+        // This is safe because we allocated the Vec with enough
+        // capacity.
+        v.extend_with_element(n, elem);
+    }
     v
 }
