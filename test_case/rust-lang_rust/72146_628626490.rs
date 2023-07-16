diff
diff --git a/src/libcore/intrinsics.rs b/src/libcore/intrinsics.rs
index 962bcbe6198..4c8650168c1 100644
--- a/src/libcore/intrinsics.rs
+++ b/src/libcore/intrinsics.rs
@@ -1954,8 +1954,13 @@ pub(crate) fn is_aligned_and_not_null<T>(ptr: *const T) -> bool {
 pub(crate) fn is_nonoverlapping<T>(src: *const T, dst: *const T, count: usize) -> bool {
     let src_usize = src as usize;
     let dst_usize = dst as usize;
-    let size = mem::size_of::<T>().checked_mul(count).unwrap();
-    let diff = if src_usize > dst_usize { src_usize - dst_usize } else { dst_usize - src_usize };
+    let size = mem::size_of::<T>().checked_mul(count).unwrap_or_else(|| unsafe { abort() });
+    // Wrapping subs here are fine because we've already checked the condition
+    let diff = if src_usize > dst_usize {
+        src_usize.wrapping_sub(dst_usize)
+    } else {
+        dst_usize.wrapping_sub(src_usize)
+    };
     // If the absolute distance between the ptrs is at least as big as the size of the buffer,
     // they do not overlap.
     diff >= size
