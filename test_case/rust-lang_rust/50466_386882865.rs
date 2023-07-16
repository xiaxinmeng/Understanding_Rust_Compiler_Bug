diff
diff --git a/src/liballoc/alloc.rs b/src/liballoc/alloc.rs
index f59c9f7fd6..7e31ee664c 100644
--- a/src/liballoc/alloc.rs
+++ b/src/liballoc/alloc.rs
@@ -152,7 +152,7 @@ unsafe fn exchange_malloc(size: usize, align: usize) -> *mut u8 {
     }
 }
 
-#[cfg(stage0)]
+#[cfg(all(stage0, not(test)))]
 #[lang = "box_free"]
 #[inline]
 unsafe fn old_box_free<T: ?Sized>(ptr: *mut T) {
