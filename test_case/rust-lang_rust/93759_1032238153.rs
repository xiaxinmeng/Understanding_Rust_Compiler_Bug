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
Diff in /checkout/compiler/rustc_ast_pretty/src/pp/convenience.rs at line 75:
 
     pub fn trailing_comma(&mut self) {
     pub fn trailing_comma(&mut self) {
-        self.scan_break(BreakToken {
-            pre_break: Some(','),
-            ..BreakToken::default()
-        });
+        self.scan_break(BreakToken { pre_break: Some(','), ..BreakToken::default() });
 
     pub fn trailing_comma_or_space(&mut self) {
     pub fn trailing_comma_or_space(&mut self) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast_pretty/src/pp/ring.rs" "/checkout/compiler/rustc_ast_pretty/src/pp/convenience.rs" "/checkout/compiler/rustc_infer/src/infer/type_variable.rs" "/checkout/compiler/rustc_ast_pretty/src/pprust/mod.rs" "/checkout/compiler/rustc_ast_pretty/src/pprust/tests.rs" "/checkout/compiler/rustc_ast_pretty/src/pprust/state/expr.rs" "/checkout/compiler/rustc_symbol_mangling/src/test.rs" "/checkout/compiler/rustc_ast_pretty/src/helpers.rs"` failed.
Build completed unsuccessfully in 0:00:14
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
