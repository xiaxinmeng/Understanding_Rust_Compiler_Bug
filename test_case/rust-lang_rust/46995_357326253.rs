patch
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index e95a5e0743..5cea0ad1fc 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -313,6 +313,8 @@ impl Step for Rustdoc {
         cargo.env("RUSTC_DEBUGINFO", builder.config.rust_debuginfo.to_string())
              .env("RUSTC_DEBUGINFO_LINES", builder.config.rust_debuginfo_lines.to_string());

+        cargo.env("LLVM_CONFIG", builder.llvm_config(target));
+
         build.run(&mut cargo);
         // Cargo adds a number of paths to the dylib search path on windows, which results in
         // the wrong rustdoc being executed. To avoid the conflicting rustdocs, we name the "tool"
