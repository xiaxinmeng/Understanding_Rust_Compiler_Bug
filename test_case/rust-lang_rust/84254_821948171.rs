diff
diff --git a/library/unwind/build.rs b/library/unwind/build.rs
index d8bf152e4d6..3bb9682a615 100644
--- a/library/unwind/build.rs
+++ b/library/unwind/build.rs
@@ -33,7 +33,15 @@ fn main() {
     } else if target.contains("solaris") {
         println!("cargo:rustc-link-lib=gcc_s");
     } else if target.contains("illumos") {
+        // Both libgcc_s and libc contain an unwinder implementation, but on
+        // some systems the libc unwinder does not contain all of the extension
+        // symbols in libgcc_s that are used by Rust.  We include libc here
+        // explicitly in an attempt to ensure it will always appear after
+        // libgcc_s in the library search order in the output object, so that we
+        // do not end up using part of an unwinder implementation from two
+        // different libraries.
         println!("cargo:rustc-link-lib=gcc_s");
+        println!("cargo:rustc-link-lib=c");
     } else if target.contains("dragonfly") {
         println!("cargo:rustc-link-lib=gcc_pic");
     } else if target.contains("pc-windows-gnu") {
