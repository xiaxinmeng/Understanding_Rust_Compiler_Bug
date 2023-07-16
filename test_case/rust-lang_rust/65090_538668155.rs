diff
diff --git a/src/elf/dynamic.rs b/src/elf/dynamic.rs
index 52ca0bd..30540a5 100644
--- a/src/elf/dynamic.rs
+++ b/src/elf/dynamic.rs
@@ -1,11 +1,9 @@

 macro_rules! elf_dyn {
     ($size:ty) => {
-        #[cfg(feature = "alloc")]
-        use scroll::{Pread, Pwrite, SizeWith};
         #[repr(C)]
         #[derive(Copy, Clone, PartialEq, Default)]
-        #[cfg_attr(feature = "alloc", derive(Pread, Pwrite, SizeWith))]
+        #[cfg_attr(feature = "alloc", derive(scroll::Pread, scroll::Pwrite, scroll::SizeWith))]
         /// An entry in the dynamic array
         pub struct Dyn {
             /// Dynamic entry type
diff --git a/src/elf/section_header.rs b/src/elf/section_header.rs
index 2eb9143..bbbe88d 100644
--- a/src/elf/section_header.rs
+++ b/src/elf/section_header.rs
@@ -1,10 +1,8 @@
 macro_rules! elf_section_header {
     ($size:ident) => {
-        #[cfg(feature = "alloc")]
-        use scroll::{Pread, Pwrite, SizeWith};
         #[repr(C)]
         #[derive(Copy, Clone, Eq, PartialEq, Default)]
-        #[cfg_attr(feature = "alloc", derive(Pread, Pwrite, SizeWith))]
+        #[cfg_attr(feature = "alloc", derive(scroll::Pread, scroll::Pwrite, scroll::SizeWith))]
         /// Section Headers are typically used by humans and static linkers for additional information or how to relocate the object
         ///
         /// **NOTE** section headers are strippable from a binary without any loss of portability/executability; _do not_ rely on them being there!
