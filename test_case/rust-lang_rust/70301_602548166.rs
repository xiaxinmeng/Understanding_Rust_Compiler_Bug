diff
diff --git a/src/liballoc/vec.rs b/src/liballoc/vec.rs
index 4769091183a..fd2b336eceb 100644
--- a/src/liballoc/vec.rs
+++ b/src/liballoc/vec.rs
@@ -2121,8 +2121,9 @@ where
         self.reserve(slice.len());
         unsafe {
             let len = self.len();
+            let dst_slice = slice::from_raw_parts_mut(self.as_mut_ptr().add(len), slice.len());
+            dst_slice.copy_from_slice(slice);
             self.set_len(len + slice.len());
-            self.get_unchecked_mut(len..).copy_from_slice(slice);
         }
     }
 }
