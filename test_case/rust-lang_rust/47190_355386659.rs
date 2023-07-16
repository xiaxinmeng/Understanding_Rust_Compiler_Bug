
--- a/src/libpanic_abort/Cargo.toml
+++ b/src/libpanic_abort/Cargo.toml
@@ -14,4 +14,4 @@ core = { path = "../libcore" }
 libc = { path = "../rustc/libc_shim" } 
 
 [target.'cfg(target_os = "cloudabi")'.dependencies]
-cloudabi = { path = "../rustc/cloudabi_shim" }
+cloudabi = { version = "0.0.2", default-features = false }
diff --git a/src/libstd/Cargo.toml b/src/libstd/Cargo.toml
index 5e4f5204aa..0b4433d879 100644
--- a/src/libstd/Cargo.toml
+++ b/src/libstd/Cargo.toml
@@ -39,7 +39,7 @@ rustc_msan = { path = "../librustc_msan" }
 rustc_tsan = { path = "../librustc_tsan" }
 
 [target.'cfg(target_os = "cloudabi")'.dependencies]
-cloudabi = { path = "../rustc/cloudabi_shim" }
+cloudabi = { version = "0.0.2", default-features = false }
 
 [build-dependencies]
 build_helper = { path = "../build_helper" }
