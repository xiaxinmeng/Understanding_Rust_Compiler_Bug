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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/doc.rs at line 23:
 use crate::util::symlink_dir;
 
 macro_rules! submodule_helper {
-    ($path:expr, submodule) => { $path };
-    ($path:expr, submodule = $submodule:literal) => { $submodule };
+    ($path:expr, submodule) => {
+        $path
+    };
+    ($path:expr, submodule = $submodule:literal) => {
+        $submodule
 }
 
 macro_rules! book {
 macro_rules! book {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/flags.rs" "/checkout/src/bootstrap/clean.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/bootstrap/toolstate.rs" "/checkout/src/bootstrap/native.rs" "/checkout/src/bootstrap/cache.rs" "/checkout/src/bootstrap/tool.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
