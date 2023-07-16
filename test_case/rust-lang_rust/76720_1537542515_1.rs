diff
diff --git a/src/bootstrap/builder.rs b/src/bootstrap/builder.rs
index 1267c0be719..875bd09dc9e 100644
--- a/src/bootstrap/builder.rs
+++ b/src/bootstrap/builder.rs
@@ -1560,10 +1560,7 @@ pub fn cargo(
         //
         // Avoid doing this during dry run as that usually means the relevant
         // compiler is not yet linked/copied properly.
-        //
-        // Only clear out the directory if we're compiling std; otherwise, we
-        // should let Cargo take care of things for us (via depdep info)
-        if !self.config.dry_run() && mode == Mode::Std && cmd == "build" {
+        if !self.config.dry_run() {
             self.clear_if_dirty(&out_dir, &self.rustc(compiler));
         }
 