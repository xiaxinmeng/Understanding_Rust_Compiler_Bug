diff
diff --git a/src/bootstrap/test.rs b/src/bootstrap/test.rs
index 86d940cd733..9a33d952ff1 100644
--- a/src/bootstrap/test.rs
+++ b/src/bootstrap/test.rs
@@ -1257,7 +1257,7 @@ fn run(self, builder: &Builder<'_>) {
             cmd.arg("--verbose");
         }
 
-        if !builder.config.verbose_tests {
+        if !builder.config.verbose_tests && !builder.is_verbose() {
             cmd.arg("--quiet");
         }
 
@@ -1808,7 +1808,7 @@ fn run(self, builder: &Builder<'_>) {
         cargo.arg("--");
         cargo.args(&builder.config.cmd.test_args());
 
-        if !builder.config.verbose_tests {
+        if !builder.config.verbose_tests && !builder.is_verbose() {
             cargo.arg("--quiet");
         }
 
@@ -1921,7 +1921,7 @@ fn run(self, builder: &Builder<'_>) {
         dylib_path.insert(0, PathBuf::from(&*builder.sysroot_libdir(compiler, target)));
         cargo.env(dylib_path_var(), env::join_paths(&dylib_path).unwrap());
 
-        if !builder.config.verbose_tests {
+        if !builder.config.verbose_tests && !builder.is_verbose() {
             cargo.arg("--quiet");
         }
 
@@ -1992,7 +1992,7 @@ fn run(self, builder: &Builder<'_>) {
             cargo.arg("'-Ctarget-feature=-crt-static'");
         }
 
-        if !builder.config.verbose_tests {
+        if !builder.config.verbose_tests && !builder.is_verbose() {
             cargo.arg("--quiet");
         }
 
