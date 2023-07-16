
diff --git a/src/librustc/driver/driver.rs b/src/librustc/driver/driver.rs
index aa3ab80..36da12a 100644
--- a/src/librustc/driver/driver.rs
+++ b/src/librustc/driver/driver.rs
@@ -779,7 +779,13 @@ pub fn build_session_options(binary: ~str,
     let sysroot_opt = matches.opt_str("sysroot").map(|m| @Path::new(m));
     let target = matches.opt_str("target").unwrap_or(host_triple());
     let target_cpu = matches.opt_str("target-cpu").unwrap_or(~"generic");
-    let target_feature = matches.opt_str("target-feature").unwrap_or(~"");
+    let mut target_feature = matches.opt_str("target-feature").unwrap_or(~"");
+    if (target == ~"arm-unknown-linux-gnueabihf") {
+        if (target_feature != ~"") {
+            target_feature.push_char(',');
+       }
+       target_feature.push_str("+vfp3");
+    }
     let save_temps = matches.opt_present("save-temps");
     let opt_level = {
         if (debugging_opts & session::no_opt) != 0 {
