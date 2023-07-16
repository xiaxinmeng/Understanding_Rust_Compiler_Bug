diff
diff --git a/library/std/src/lib.rs b/library/std/src/lib.rs
index 90603cd9836..1f15af0c58b 100644
--- a/library/std/src/lib.rs
+++ b/library/std/src/lib.rs
@@ -348,15 +348,19 @@
 #[allow(unused_imports)] // macros from `alloc` are not used on all platforms
 #[macro_use]
 extern crate alloc as alloc_crate;
-#[doc(masked)]
-#[allow(unused_extern_crates)]
-extern crate libc;

 // We always need an unwinder currently for backtraces
 #[doc(masked)]
 #[allow(unused_extern_crates)]
 extern crate unwind;

+// On some platforms an unwinder is present in both libc and another library;
+// e.g., in libgcc_s on illumos.  Depend on libc after unwind in order to arrive
+// at the correct library search order in the output object; i.e., libc last.
+#[doc(masked)]
+#[allow(unused_extern_crates)]
+extern crate libc;
+
 // During testing, this crate is not actually the "real" std library, but rather
 // it links to the real std library, which was compiled from this same source
 // code. So any lang items std defines are conditionally excluded (or else they
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
