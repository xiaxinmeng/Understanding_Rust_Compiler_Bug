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
Diff in /checkout/src/librustdoc/passes/unindent_comments/tests.rs at line 1:
 use super::*;
+use rustc_span::create_default_session_globals_then;
 use rustc_span::source_map::DUMMY_SP;
 use rustc_span::symbol::Symbol;
-use rustc_span::create_default_session_globals_then;
 
 fn create_doc_fragment(s: &str) -> Vec<DocFragment> {
     vec![DocFragment {
Diff in /checkout/src/librustdoc/clean/cfg/tests.rs at line 2:
 use rustc_ast::attr;
 use rustc_ast::Path;
 use rustc_ast::Path;
-use rustc_span::symbol::{Ident, Symbol};
 use rustc_span::create_default_session_globals_then;
+use rustc_span::symbol::{Ident, Symbol};
 use rustc_span::DUMMY_SP;
 
 fn word_cfg(s: &str) -> Cfg {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/passes/unindent_comments.rs" "/checkout/src/librustdoc/passes/strip_hidden.rs" "/checkout/src/librustdoc/lib.rs" "/checkout/src/librustdoc/passes/unindent_comments/tests.rs" "/checkout/src/bootstrap/check.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/html/mod.rs" "/checkout/src/librustdoc/passes/check_code_block_syntax.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
