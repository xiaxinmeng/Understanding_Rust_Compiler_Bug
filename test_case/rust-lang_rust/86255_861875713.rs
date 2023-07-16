diff
diff --git a/compiler/rustc_middle/src/mir/interpret/allocation.rs b/compiler/rustc_middle/src/mir/interpret/allocation.rs
index 7405a70d39a..71d09e16ec4 100644
--- a/compiler/rustc_middle/src/mir/interpret/allocation.rs
+++ b/compiler/rustc_middle/src/mir/interpret/allocation.rs
@@ -125,6 +125,9 @@ pub fn from_bytes_byte_aligned_immutable<'a>(slice: impl Into<Cow<'a, [u8]>>) ->
     /// Try to create an Allocation of `size` bytes, failing if there is not enough memory
     /// available to the compiler to do so.
     pub fn uninit(size: Size, align: Align) -> InterpResult<'static, Self> {
+        if std::env::var(std::ffi::OsStr::new("RUSTC_ALWAYS_FAIL_MIRI_ALLOC")).is_ok() {
+            Err(InterpError::ResourceExhaustion(ResourceExhaustionInfo::MemoryExhausted))?;
+        }
         let mut bytes = Vec::new();
         bytes.try_reserve(size.bytes_usize()).map_err(|_| {
