plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/util/tests.rs"` failed.
Diff in /checkout/src/tools/compiletest/src/util/tests.rs at line 39:
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 #[test]
 #[test]
 fn path_buf_with_extra_extension_test() {
-    assert_eq!(PathBuf::from("foo.rs.stderr"), PathBuf::from("foo.rs").with_extra_extension("stderr"));
-    assert_eq!(PathBuf::from("foo.rs.stderr"), PathBuf::from("foo.rs").with_extra_extension(".stderr"));
+    assert_eq!(
+        PathBuf::from("foo.rs.stderr"),
+        PathBuf::from("foo.rs").with_extra_extension("stderr")
+    );
+    assert_eq!(
+        PathBuf::from("foo.rs.stderr"),
+        PathBuf::from("foo.rs").with_extra_extension(".stderr")
+    );
     assert_eq!(PathBuf::from("foo.rs"), PathBuf::from("foo.rs").with_extra_extension(""));
 
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:18
