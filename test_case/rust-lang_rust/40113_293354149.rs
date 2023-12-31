
--- a/src/liblibc/src/unix/mod.rs
+++ b/src/liblibc/src/unix/mod.rs
@@ -210,8 +210,7 @@ cfg_if! {
         // cargo build, don't pull in anything extra as the libstd  dep
         // already pulls in all libs.
-    } else if #[cfg(any(all(target_env = "musl", not(target_arch = "mips"))))] {
+    } else if #[cfg(target_env = "musl")] {
-        #[link(name = "c", kind = "static", cfg(target_feature = "crt-static"))]
-        #[link(name = "c", cfg(not(target_feature = "crt-static")))]
+        #[link(name = "c")]
         extern {}
     } else if #[cfg(target_os = "emscripten")] {
         #[link(name = "c")]
