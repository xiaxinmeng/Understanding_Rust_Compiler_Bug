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
Diff in /checkout/src/bootstrap/doc.rs at line 676:
 }
 
 tool_doc!(Rustdoc, "rustdoc-tool", "src/tools/rustdoc", ["rustdoc", "rustdoc-json-types"]);
-tool_doc!(Rustfmt, "rustfmt-nightly", "src/tools/rustfmt", ["rustfmt-nightly", "rustfmt-config_proc_macro"], binary=true);
+tool_doc!(
+    "rustfmt-nightly",
+    "src/tools/rustfmt",
+    "src/tools/rustfmt",
+    ["rustfmt-nightly", "rustfmt-config_proc_macro"],
+    binary = true
 
 
 #[derive(Ord, PartialOrd, Debug, Copy, Clone, Hash, PartialEq, Eq)]
 pub struct ErrorIndex {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/setup.rs" "/checkout/src/bootstrap/tool.rs" "/checkout/src/bootstrap/flags.rs" "/checkout/src/bootstrap/clean.rs" "/checkout/src/bootstrap/doc.rs" "/checkout/src/bootstrap/builder/tests.rs" "/checkout/src/bootstrap/toolstate.rs" "/checkout/src/bootstrap/channel.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:12
