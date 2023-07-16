diff
diff --git a/src/librustc_llvm/build.rs b/src/librustc_llvm/build.rs
index 21b8080714c..d8a2d302168 100644
--- a/src/librustc_llvm/build.rs
+++ b/src/librustc_llvm/build.rs
@@ -189,6 +189,10 @@ fn main() {

     if !is_crossed {
         cmd.arg("--system-libs");
+    } else if target.contains("haiku") {
+        // LLVM is built with LLVM_ENABLE_ZLIB as on, which means it should
+        // link to libz.so on Haiku.
+        println!("cargo:rustc-link-lib=z");
     }
     cmd.args(&components);
