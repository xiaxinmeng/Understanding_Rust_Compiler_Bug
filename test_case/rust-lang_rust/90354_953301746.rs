plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/src/bootstrap/doc.rs at line 755:
     "src/tools/rustfmt",
     ["rustfmt-nightly", "rustfmt-config_proc_macro"],
-tool_doc!(
-    Clippy,
-    "clippy",
-    "src/tools/clippy",
-    "src/tools/clippy",
-    ["clippy_lints", "clippy_utils"],
-);
+tool_doc!(Clippy, "clippy", "src/tools/clippy", ["clippy_lints", "clippy_utils"],);
 
 #[derive(Ord, PartialOrd, Debug, Copy, Clone, Hash, PartialEq, Eq)]
 pub struct ErrorIndex {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/format.rs" "/checkout/src/bootstrap/check.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/src/bootstrap/toolstate.rs" "/checkout/src/bootstrap/cache.rs" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/tools/remote-test-client/src/main.rs" "/checkout/src/bootstrap/clean.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
