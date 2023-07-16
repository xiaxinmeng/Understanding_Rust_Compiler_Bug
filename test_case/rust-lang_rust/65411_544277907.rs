diff
diff --git a/src/tools/compiletest/src/runtest.rs b/src/tools/compiletest/src/runtest.rs
index ea31f37c7a5..66546c5ca0c 100644
--- a/src/tools/compiletest/src/runtest.rs
+++ b/src/tools/compiletest/src/runtest.rs
@@ -2869,6 +2869,7 @@ impl<'test> TestCx<'test> {
         // We don't want RUSTFLAGS set from the outside to interfere with
         // compiler flags set in the test cases:
         cmd.env_remove("RUSTFLAGS");
+        cmd.env("RUSTFLAGS", "-Zsave-analysis");
 
         // Use dynamic musl for tests because static doesn't allow creating dylibs
         if self.config.host.contains("musl") {
