diff
diff --git a/src/bootstrap/builder.rs b/src/bootstrap/builder.rs
index 1267c0be719..d64c4fa16c6 100644
--- a/src/bootstrap/builder.rs
+++ b/src/bootstrap/builder.rs
@@ -1563,7 +1563,7 @@ pub fn cargo(
         //
         // Only clear out the directory if we're compiling std; otherwise, we
         // should let Cargo take care of things for us (via depdep info)
-        if !self.config.dry_run() && mode == Mode::Std && cmd == "build" {
+        if !self.config.dry_run() && mode == Mode::Std {
             self.clear_if_dirty(&out_dir, &self.rustc(compiler));
         }
 