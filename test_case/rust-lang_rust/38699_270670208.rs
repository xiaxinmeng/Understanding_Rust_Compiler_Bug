 diff
--- a/src/libstd/Cargo.toml
+++ b/src/libstd/Cargo.toml
@@ -7,7 +7,6 @@ build = "build.rs"
 [lib]
 name = "std"
 path = "lib.rs"
-crate-type = ["dylib", "rlib"]

 [dependencies]
 alloc = { path = "../liballoc" }
