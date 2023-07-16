diff
diff --git a/src/librustdoc/doctest.rs b/src/librustdoc/doctest.rs
index de4a3732e73..56ccdfae1d8 100644
--- a/src/librustdoc/doctest.rs
+++ b/src/librustdoc/doctest.rs
@@ -208,29 +205,19 @@
     Ok(())
 }
 
-crate fn run_tests(
-    mut test_args: Vec<String>,
-    nocapture: bool,
-    display_doctest_warnings: bool,
-    tests: Vec<test::TestDescAndFn>,
-) {
+crate fn run_tests(mut test_args: Vec<String>, nocapture: bool, tests: Vec<test::TestDescAndFn>) {
     test_args.insert(0, "rustdoctest".to_string());
     if nocapture {
         test_args.push("--nocapture".to_string());
     }
-    test::test_main(
-        &test_args,
-        tests,
-        Some(test::Options::new().display_output(display_doctest_warnings)),
-    );
+    test::test_main(&test_args, tests, None);
 }
 
