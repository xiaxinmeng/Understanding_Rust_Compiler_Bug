diff
diff --git a/src/lib.rs b/src/lib.rs
index 6a9081f..a2b610a 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1714,6 +1714,8 @@ impl Build {
                     } else {
                         clang_compiler
                     }
+                } else if target.contains("wasm32-unknown-cloudabi") {
+                     default.to_string()
                 } else if target.contains("cloudabi") {
                     format!("{}-{}", target, traditional)
                 } else if self.get_host()? != target {
