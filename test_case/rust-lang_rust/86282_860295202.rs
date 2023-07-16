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
Diff in /checkout/src/librustdoc/clean/utils.rs at line 553:
 pub(super) fn render_macro_arms(matchers: &[TokenTree]) -> String {
         .iter()
         .iter()
-        .map(|matcher| {
-            format!("    {} => {{ ... }};\n", render_macro_matcher(matcher))
-        })
+        .map(|matcher| format!("    {} => {{ ... }};\n", render_macro_matcher(matcher)))
         .collect::<String>()
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/json/mod.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/clean/cfg/tests.rs" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/clean/auto_trait.rs" "/checkout/src/librustdoc/json/conversions.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
