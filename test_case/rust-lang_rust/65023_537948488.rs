
diff --git a/src/libcore/slice/mod.rs b/src/libcore/slice/mod.rs
index b5462d98837..e55baded913 100644
--- a/src/libcore/slice/mod.rs
+++ b/src/libcore/slice/mod.rs
@@ -64,6 +64,7 @@ impl<T> [T] {
     #[inline]
     // SAFETY: const sound because we transmute out the length field as a usize (which it must be)
     #[allow_internal_unstable(const_fn_union)]
+    #[allow(unused_attributes)]
     pub const fn len(&self) -> usize {
         unsafe {
             crate::ptr::Repr { rust: self }.raw.len
diff --git a/src/libcore/str/mod.rs b/src/libcore/str/mod.rs
index ece61dde490..d02d232a20e 100644
--- a/src/libcore/str/mod.rs
+++ b/src/libcore/str/mod.rs
@@ -2168,6 +2168,7 @@ impl str {
     #[inline(always)]
     // SAFETY: const sound because we transmute two types with the same layout
     #[allow_internal_unstable(const_fn_union)]
+    #[allow(unused_attributes)]
     pub const fn as_bytes(&self) -> &[u8] {
         #[repr(C)]
         union Slices<'a> {
