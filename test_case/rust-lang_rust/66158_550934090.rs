diff
diff --git a/src/bootstrap/test.rs b/src/bootstrap/test.rs
index 60f0dccdb07..a858ed42bad 100644
--- a/src/bootstrap/test.rs
+++ b/src/bootstrap/test.rs
@@ -570,7 +570,12 @@ impl Step for Clippy {
             let host_libs = builder
                 .stage_out(compiler, Mode::ToolRustc)
                 .join(builder.cargo_dir());
+            let target_libs = builder
+                .stage_out(compiler, Mode::ToolRustc)
+                .join(&self.host)
+                .join(builder.cargo_dir());
             cargo.env("HOST_LIBS", host_libs);
+            cargo.env("TARGET_LIBS", target_libs);
             // clippy tests need to find the driver
             cargo.env("CLIPPY_DRIVER_PATH", clippy);
 
