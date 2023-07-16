diff
--- a/src/bootstrap/native.rs
+++ b/src/bootstrap/native.rs
@@ -568,7 +568,7 @@ fn configure_cmake(
     // We also do this if the user explicitly requested static libstdc++.
     if builder.config.llvm_static_stdcpp {
         if !target.contains("msvc") && !target.contains("netbsd") {
-            if target.contains("apple") {
+            if target.contains("apple") || target.contains("windows") {
                 ldflags.push_all("-static-libstdc++");
             } else {
                 ldflags.push_all("-Wl,-Bsymbolic -static-libstdc++");
