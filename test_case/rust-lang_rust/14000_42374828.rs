 diff
﻿﻿﻿﻿diff --git a/src/librustdoc/test.rs b/src/librustdoc/test.rs
index 01cbd15..481fee3 100644
--- a/src/librustdoc/test.rs
+++ b/src/librustdoc/test.rs
@@ -144,12 +144,15 @@ fn runtest(test: &str, cratename: &str, libs: HashSet<Path>, should_fail: bool,
     let outdir = TempDir::new("rustdoctest").expect("rustdoc needs a tempdir");
     let out = Some(outdir.path().clone());
     let cfg = driver::build_configuration(&sess);
+    let path = sess.target_filesearch().get_lib_path();
     driver::compile_input(sess, cfg, &input, &out, &None);

     if no_run { return }

     // Run the code!
     let exe = outdir.path().join("rust_out");
+    ::std::unstable::dynamic_lib::DynamicLibrary::add_search_path(
+        &path);
     let out = Process::output(exe.as_str().unwrap(), []);
     match out {
         Err(e) => fail!("couldn't run the test: {}{}", e,
