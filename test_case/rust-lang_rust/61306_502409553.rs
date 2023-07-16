
diff --git a/src/librustc_codegen_llvm/llvm/ffi.rs b/src/librustc_codegen_llvm/llvm/ffi.rs
index a71243c7c8..a5c295cd45 100644
--- a/src/librustc_codegen_llvm/llvm/ffi.rs
+++ b/src/librustc_codegen_llvm/llvm/ffi.rs
@@ -564,7 +564,7 @@ pub mod debuginfo {
 
     // These values **must** match with LLVMRustDIFlags!!
     bitflags! {
-        #[repr(C)]
+        #[repr(transparent)]
         #[derive(Default)]
         pub struct DIFlags: ::libc::uint32_t {
             const FlagZero                = 0;
@@ -593,7 +593,7 @@ pub mod debuginfo {
 
     // These values **must** match with LLVMRustDISPFlags!!
     bitflags! {
-        #[repr(C)]
+        #[repr(transparent)]
         #[derive(Default)]
         pub struct DISPFlags: ::libc::uint32_t {
             const SPFlagZero              = 0;
