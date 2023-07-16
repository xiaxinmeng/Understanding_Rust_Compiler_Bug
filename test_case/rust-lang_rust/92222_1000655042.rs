
diff --git a/compiler/rustc/src/main.rs b/compiler/rustc/src/main.rs
index c80fab99496..2d8cfa2531b 100644
--- a/compiler/rustc/src/main.rs
+++ b/compiler/rustc/src/main.rs
@@ -11,6 +11,34 @@
 #[cfg(feature = "tikv-jemalloc-sys")]
 use tikv_jemalloc_sys as jemalloc_sys;

+use std::alloc::{GlobalAlloc, Layout};
+use tikv_jemallocator::Jemalloc;
+
+#[no_mangle]
+pub unsafe extern "C" fn __rust_alloc(size: usize, align: usize) -> *mut u8 {
+    Jemalloc.alloc(Layout::from_size_align_unchecked(size, align))
+}
+
+#[no_mangle]
+pub unsafe extern "C" fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
+    Jemalloc.alloc_zeroed(Layout::from_size_align_unchecked(size, align))
+}
+
+#[no_mangle]
+pub unsafe extern "C" fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize) {
+    Jemalloc.dealloc(ptr, Layout::from_size_align_unchecked(size, align))
+}
+
+#[no_mangle]
+pub unsafe extern "C" fn __rust_realloc(
+    ptr: *mut u8,
+    old_size: usize,
+    align: usize,
+    new_size: usize,
+) -> *mut u8 {
+    Jemalloc.realloc(ptr, Layout::from_size_align_unchecked(old_size, align), new_size)
+}
+
 fn main() {
     // Pull in jemalloc when enabled.
     //
