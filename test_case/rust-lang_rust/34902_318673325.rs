diff
--- a/src/librustc_llvm/build.rs
+++ b/src/librustc_llvm/build.rs
@@ -137,6 +137,11 @@
     let cxxflags = output(&mut cmd);
     let mut cfg = gcc::Config::new();
     for flag in cxxflags.split_whitespace() {
+        // Split-dwarf gives unreproducible DW_AT_GNU_dwo_id so don't do it
+        if flag == "-gsplit-dwarf" {
+            continue;
+        }
+
         // Ignore flags like `-m64` when we're doing a cross build
         if is_crossed && flag.starts_with("-m") {
             continue;
