diff
diff --git a/src/bootstrap/test.rs b/src/bootstrap/test.rs
index 60f0dccdb07..76a5c88b5f0 100644
--- a/src/bootstrap/test.rs
+++ b/src/bootstrap/test.rs
@@ -569,6 +569,7 @@ impl Step for Clippy {
             cargo.env("RUSTC_LIB_PATH", builder.rustc_libdir(compiler));
             let host_libs = builder
                 .stage_out(compiler, Mode::ToolRustc)
+                .join(&self.host)
                 .join(builder.cargo_dir());
             cargo.env("HOST_LIBS", host_libs);
             // clippy tests need to find the driver
