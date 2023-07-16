console
$ git diff
diff --git a/Cargo.lock b/Cargo.lock
index e45926f832c..6633d667dce 100644
--- a/Cargo.lock
+++ b/Cargo.lock
@@ -2506,6 +2506,12 @@ version = "2.1.0"
 source = "registry+https://github.com/rust-lang/crates.io-index"
 checksum = "d4fd5641d01c8f18a23da7b6fe29298ff4b55afcccdf78973b24cf3175fee32e"
 
+[[package]]
+name = "peresil"
+version = "0.3.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "f658886ed52e196e850cfbbfddab9eaa7f6d90dd0929e264c31e5cec07e09e57"
+
 [[package]]
 name = "perf-event-open-sys"
 version = "1.0.1"
@@ -4590,6 +4596,19 @@ dependencies = [
  "serde_json",
 ]
 
+[[package]]
+name = "rustdoc-test"
+version = "0.1.0"
+dependencies = [
+ "fs-err",
+ "getopts",
+ "once_cell",
+ "regex",
+ "shlex",
+ "sxd-document",
+ "sxd-xpath",
+]
+
 [[package]]
 name = "rustdoc-themes"
 version = "0.1.0"
@@ -5078,6 +5097,27 @@ dependencies = [
  "syn",
 ]
 
+[[package]]
+name = "sxd-document"
+version = "0.3.2"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "94d82f37be9faf1b10a82c4bd492b74f698e40082f0f40de38ab275f31d42078"
+dependencies = [
+ "peresil",
+ "typed-arena",
+]
+
+[[package]]
+name = "sxd-xpath"
+version = "0.4.2"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "36e39da5d30887b5690e29de4c5ebb8ddff64ebd9933f98a01daaa4fd11b36ea"
+dependencies = [
+ "peresil",
+ "quick-error 1.2.3",
+ "sxd-document",
+]
+
 [[package]]
 name = "syn"
 version = "1.0.65"
@@ -5457,6 +5497,12 @@ dependencies = [
  "tracing-subscriber",
 ]
 
+[[package]]
+name = "typed-arena"
+version = "1.7.0"
+source = "registry+https://github.com/rust-lang/crates.io-index"
+checksum = "a9b2228007eba4120145f785df0f6c92ea538f5a3635a612ecf4e334c8c1446d"
+
 [[package]]
 name = "typenum"
 version = "1.12.0"
diff --git a/Cargo.toml b/Cargo.toml
index 42dd5d7ef43..6583375600b 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -37,6 +37,7 @@ members = [
   "src/tools/html-checker",
   "src/tools/bump-stage0",
   "src/tools/lld-wrapper",
+  "src/tools/rustdoc-test",
 ]
 
 exclude = [
diff --git a/library/backtrace b/library/backtrace
--- a/library/backtrace
+++ b/library/backtrace
@@ -1 +1 @@
-Subproject commit cc89bb66f91b2b4a640b0b525ca5d753e3346d7e
+Subproject commit cc89bb66f91b2b4a640b0b525ca5d753e3346d7e-dirty
diff --git a/library/stdarch b/library/stdarch
--- a/library/stdarch
+++ b/library/stdarch
@@ -1 +1 @@
-Subproject commit 5fdbc476afc81a789806697fc4a2d9d19b8c9993
+Subproject commit 5fdbc476afc81a789806697fc4a2d9d19b8c9993-dirty
diff --git a/src/bootstrap/test.rs b/src/bootstrap/test.rs
index 8594fa42266..a646e5dd5f4 100644
--- a/src/bootstrap/test.rs
+++ b/src/bootstrap/test.rs
@@ -1294,6 +1294,12 @@ fn run(self, builder: &Builder<'_>) {
             cmd.arg("--jsondocck-path")
                 .arg(builder.ensure(tool::JsonDocCk { compiler: json_compiler, target }));
         }
+        if mode == "rustdoc" {
+         // Use the beta compiler for rustdoc-test
+            let rustdoc_test_compiler = compiler.with_stage(0);
+            cmd.arg("--rustdoc-test-path")
+                .arg(builder.ensure(tool::RustdocTest { compiler: rustdoc_test_compiler, target }));
+        }
 
         if mode == "run-make" && suite.ends_with("fulldeps") {
             let rust_demangler = builder
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index af6f4bb0e5f..97e9dad0b12 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -376,6 +376,7 @@ fn run(self, builder: &Builder<'_>) -> PathBuf {
     ExpandYamlAnchors, "src/tools/expand-yaml-anchors", "expand-yaml-anchors";
     LintDocs, "src/tools/lint-docs", "lint-docs";
     JsonDocCk, "src/tools/jsondocck", "jsondocck";
+    RustdocTest, "src/tools/rustdoc-test", "rustdoc-test";
     HtmlChecker, "src/tools/html-checker", "html-checker";
     BumpStage0, "src/tools/bump-stage0", "bump-stage0";
 );
diff --git a/src/tools/compiletest/src/common.rs b/src/tools/compiletest/src/common.rs
index cd0a56d08d8..029f23f0057 100644
--- a/src/tools/compiletest/src/common.rs
+++ b/src/tools/compiletest/src/common.rs
@@ -207,6 +207,9 @@ pub struct Config {
     /// The jsondocck executable.
     pub jsondocck_path: Option<String>,
 
+    /// The rustdoc-test executable.
+    pub rustdoc_test_path: Option<String>,
+
     /// The LLVM `FileCheck` binary path.
     pub llvm_filecheck: Option<PathBuf>,
 
diff --git a/src/tools/compiletest/src/main.rs b/src/tools/compiletest/src/main.rs
index 87aba8c5d32..07aa2b4ecb1 100644
--- a/src/tools/compiletest/src/main.rs
+++ b/src/tools/compiletest/src/main.rs
@@ -64,6 +64,7 @@ pub fn parse_config(args: Vec<String>) -> Config {
         .reqopt("", "lldb-python", "path to python to use for doc tests", "PATH")
         .reqopt("", "docck-python", "path to python to use for doc tests", "PATH")
         .optopt("", "jsondocck-path", "path to jsondocck to use for doc tests", "PATH")
+        .optopt("", "rustdoc-test-path", "path to rustdoc-test to use for rustdoc tests", "PATH")
         .optopt("", "valgrind-path", "path to Valgrind executable for Valgrind tests", "PROGRAM")
         .optflag("", "force-valgrind", "fail if Valgrind tests cannot be run under Valgrind")
         .optopt("", "run-clang-based-tests-with", "path to Clang executable", "PATH")
@@ -223,6 +224,7 @@ fn make_absolute(path: PathBuf) -> PathBuf {
         lldb_python: matches.opt_str("lldb-python").unwrap(),
         docck_python: matches.opt_str("docck-python").unwrap(),
         jsondocck_path: matches.opt_str("jsondocck-path"),
+        rustdoc_test_path: matches.opt_str("rustdoc-test-path"),
         valgrind_path: matches.opt_str("valgrind-path"),
         force_valgrind: matches.opt_present("force-valgrind"),
         run_clang_based_tests_with: matches.opt_str("run-clang-based-tests-with"),
diff --git a/src/tools/compiletest/src/runtest.rs b/src/tools/compiletest/src/runtest.rs
index 4470272a9f8..e75850466f1 100644
--- a/src/tools/compiletest/src/runtest.rs
+++ b/src/tools/compiletest/src/runtest.rs
@@ -2220,11 +2220,18 @@ fn run_rustdoc_test(&self) {
             self.check_rustdoc_test_option(proc_res);
         } else {
             let root = self.config.find_rust_src_root().unwrap();
+            // let res = self.cmd2procres(
+            //     Command::new(&self.config.docck_python)
+            //         .arg(root.join("src/etc/htmldocck.py"))
+            //         .arg(&out_dir)
+            //         .arg(&self.testpaths.file),
+            // );
             let res = self.cmd2procres(
-                Command::new(&self.config.docck_python)
-                    .arg(root.join("src/etc/htmldocck.py"))
-                    .arg(&out_dir)
-                    .arg(&self.testpaths.file),
+            Command::new(self.config.rustdoc_test_path.as_ref().unwrap())
+                .arg("--doc-dir")
+                .arg(root.join(&out_dir))
+                .arg("--template")
+                .arg(&self.testpaths.file),
             );
             if !res.status.success() {
                 self.fatal_proc_rec_with_ctx("htmldocck failed!", &res, |mut this| {
