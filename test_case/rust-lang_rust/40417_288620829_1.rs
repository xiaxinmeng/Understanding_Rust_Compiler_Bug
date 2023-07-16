diff
diff --git a/src/bootstrap/step.rs b/src/bootstrap/step.rs
index 6eb12fe..174c71b 100644
--- a/src/bootstrap/step.rs
+++ b/src/bootstrap/step.rs
@@ -491,7 +491,7 @@ pub fn build_rules<'a>(build: &'a Build) -> Rules {

     rules.build("test-helpers", "src/rt/rust_test_helpers.c")
          .run(move |s| native::test_helpers(build, s.target));
-    rules.build("openssl", "path/to/nowhere")
+    rules.build("openssl", "openssl")
          .run(move |s| native::openssl(build, s.target));

     // Some test suites are run inside emulators, and most of our test binaries
