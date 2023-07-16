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
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/toolstate.rs"` failed.
Diff in /checkout/src/bootstrap/toolstate.rs at line 85:
 // We do require that we checked whether they build or not on the tools builder,
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
 // though, as otherwise we will be unable to file an issue if they start
 // failing.
-static NIGHTLY_TOOLS: &[(&str, &str)] = &[
-    ("miri", "src/tools/miri"),
-    ("embedded-book", "src/doc/embedded-book"),
-];
+static NIGHTLY_TOOLS: &[(&str, &str)] =
+    &[("miri", "src/tools/miri"), ("embedded-book", "src/doc/embedded-book")];
 
 fn print_error(tool: &str, submodule: &str) {
     eprintln!();
Build completed unsuccessfully in 0:00:23
