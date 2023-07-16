
diff --git a/src/librustc_codegen_llvm/llvm/ffi.rs b/src/librustc_codegen_llvm/llvm/ffi.rs
index a71243c7c8..5ef0e6246b 100644
--- a/src/librustc_codegen_llvm/llvm/ffi.rs
+++ b/src/librustc_codegen_llvm/llvm/ffi.rs
@@ -566,6 +566,7 @@ pub mod debuginfo {
     bitflags! {
         #[repr(C)]
         #[derive(Default)]
+        #[repr(transparent)]
         pub struct DIFlags: ::libc::uint32_t {
             const FlagZero                = 0;
             const FlagPrivate             = 1;
@@ -595,6 +596,7 @@ pub mod debuginfo {
     bitflags! {
         #[repr(C)]
         #[derive(Default)]
+        #[repr(transparent)]
         pub struct DISPFlags: ::libc::uint32_t {
             const SPFlagZero              = 0;
             const SPFlagVirtual           = 1;
