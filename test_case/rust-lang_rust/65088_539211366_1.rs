diff
diff --git a/src/bootstrap/builder.rs b/src/bootstrap/builder.rs
index 5d586f0c461..ef01c18eaa0 100644
--- a/src/bootstrap/builder.rs
+++ b/src/bootstrap/builder.rs
@@ -776,7 +776,8 @@ impl<'a> Builder<'a> {
         cargo
             .env("CARGO_TARGET_DIR", out_dir)
             .arg(cmd)
-            .arg("-Zconfig-profile");
+            .arg("-Zconfig-profile")
+            .arg("-Ztimings");

         let profile_var = |name: &str| {
             let profile = if self.config.rust_optimize {
