diff
diff --git a/src/Cargo.lock b/src/Cargo.lock
index bc1fdf40b0..e5024f0427 100644
--- a/src/Cargo.lock
+++ b/src/Cargo.lock
@@ -1858,6 +1858,7 @@ dependencies = [
 name = "rustdoc-tool"
 version = "0.0.0"
 dependencies = [
+ "rustc_llvm 0.0.0",
  "rustdoc 0.0.0",
 ]
 
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index a05e58e6a2..e169e8bf12 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -313,6 +313,10 @@ impl Step for Rustdoc {
         cargo.env("RUSTC_DEBUGINFO", builder.config.rust_debuginfo.to_string())
              .env("RUSTC_DEBUGINFO_LINES", builder.config.rust_debuginfo_lines.to_string());
 
+        if build.config.llvm_link_shared {
+            cargo.env("LLVM_LINK_SHARED", "1");
+        }
+
         build.run(&mut cargo);
         // Cargo adds a number of paths to the dylib search path on windows, which results in
         // the wrong rustdoc being executed. To avoid the conflicting rustdocs, we name the "tool"
diff --git a/src/tools/rustdoc/Cargo.toml b/src/tools/rustdoc/Cargo.toml
index 344f617ef9..110f01b1bc 100644
--- a/src/tools/rustdoc/Cargo.toml
+++ b/src/tools/rustdoc/Cargo.toml
@@ -12,3 +12,4 @@ path = "main.rs"
 
 [dependencies]
 rustdoc = { path = "../../librustdoc" }
+rustc_llvm = { path = "../../librustc_llvm" }
diff --git a/src/tools/rustdoc/main.rs b/src/tools/rustdoc/main.rs
index 9c37e249ba..fd37235722 100644
--- a/src/tools/rustdoc/main.rs
+++ b/src/tools/rustdoc/main.rs
@@ -9,5 +9,6 @@
 // except according to those terms.
 
 extern crate rustdoc;
+extern crate rustc_llvm;
 
 fn main() { rustdoc::main() }
