diff
diff --git a/src/libcore/mem/mod.rs b/src/libcore/mem/mod.rs
index 770d1ca8e7..487785b878 100644
--- a/src/libcore/mem/mod.rs
+++ b/src/libcore/mem/mod.rs
@@ -450,8 +450,7 @@ pub const fn needs_drop<T>() -> bool {
 #[inline]
 #[stable(feature = "rust1", since = "1.0.0")]
 pub unsafe fn zeroed<T>() -> T {
-    intrinsics::panic_if_uninhabited::<T>();
-    intrinsics::init()
+    MaybeUninit::zeroed().assert_init()
 }
 
 /// Bypasses Rust's normal memory-initialization checks by pretending to
@@ -476,8 +475,7 @@ pub unsafe fn zeroed<T>() -> T {
 #[rustc_deprecated(since = "1.38.0", reason = "use `mem::MaybeUninit` instead")]
 #[stable(feature = "rust1", since = "1.0.0")]
 pub unsafe fn uninitialized<T>() -> T {
-    intrinsics::panic_if_uninhabited::<T>();
-    intrinsics::uninit()
+    MaybeUninit::uninit().assert_init()
 }
 
 /// Swaps the values at two mutable locations, without deinitializing either one.
