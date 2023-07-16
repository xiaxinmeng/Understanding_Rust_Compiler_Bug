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
Diff in /checkout/src/tools/compiletest/src/common.rs at line 376:
     testpaths.file.with_extension(extension)
 
 
-pub const UI_EXTENSIONS: &[&str] = &[UI_STDERR, UI_STDOUT, UI_FIXED, UI_RUN_STDERR, UI_RUN_STDOUT, UI_STDERR_64, UI_STDERR_32, UI_STDERR_16];
+pub const UI_EXTENSIONS: &[&str] = &[
+    UI_STDERR,
+    UI_STDOUT,
+    UI_FIXED,
+    UI_RUN_STDERR,
+    UI_RUN_STDOUT,
+    UI_STDERR_64,
+    UI_STDERR_32,
+    UI_STDERR_16,
+];
 pub const UI_STDERR: &str = "stderr";
 pub const UI_STDOUT: &str = "stdout";
 pub const UI_FIXED: &str = "fixed";
Build completed unsuccessfully in 0:00:14
Build completed unsuccessfully in 0:00:14
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/tools/compiletest/src/read2.rs" "/checkout/src/tools/compiletest/src/common.rs" "/checkout/src/tools/compiletest/src/tests.rs" "/checkout/src/tools/unicode-table-generator/src/skiplist.rs" "/checkout/src/tools/compiletest/src/util.rs" "/checkout/src/tools/unicode-table-generator/src/unicode_download.rs" "/checkout/src/tools/unicode-table-generator/src/case_mapping.rs" "/checkout/src/tools/compiletest/src/errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
