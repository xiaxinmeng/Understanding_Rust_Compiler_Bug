patch
diff --git a/src/librustdoc/test.rs b/src/librustdoc/test.rs
index f27d6c3597e..e5b38a8f84f 100644
--- a/src/librustdoc/test.rs
+++ b/src/librustdoc/test.rs
@@ -199,7 +199,7 @@ fn run_test(
         _ => PathBuf::from(r"doctest.rs"),
     };

-    let file_and_line = format!("{:?}:{}",path,line as isize - line_offset as isize);
+    let file_and_line = format!("{}:{}", path.display(), line as isize - line_offset as isize);

     enum DirState {
         Temp(tempfile::TempDir),
@@ -245,7 +245,6 @@ fn run_test(
         compiler.arg("--sysroot").arg(sysroot);
     }
     compiler.arg("--edition").arg(&edition.to_string());
-    compiler.current_dir(outdir.path());
     compiler.env("UNSTABLE_RUSTDOC_TEST_FILE_AND_LINE", file_and_line);
     compiler.arg("-o").arg(&output_file);
     if as_test_harness {
