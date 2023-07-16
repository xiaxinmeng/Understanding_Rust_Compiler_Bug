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
Diff in /checkout/compiler/rustc_ast_pretty/src/pprust/tests.rs at line 1:
 use super::*;
 use rustc_ast as ast;
-use rustc_span::symbol::Ident;
-use rustc_span::symbol::Ident;
 use rustc_span::create_default_session_globals_then;
+use rustc_span::symbol::Ident;
 fn fun_to_string(
 fn fun_to_string(
     decl: &ast::FnDecl,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_session/src/output.rs" "/checkout/compiler/rustc/src/main.rs" "/checkout/compiler/rustc_ast_pretty/src/pp.rs" "/checkout/compiler/rustc_ast_pretty/src/lib.rs" "/checkout/compiler/rustc_ast_pretty/src/pprust/tests.rs" "/checkout/compiler/rustc_interface/src/callbacks.rs" "/checkout/compiler/rustc_ast_pretty/src/pprust/mod.rs" "/checkout/compiler/rustc_session/src/cgu_reuse_tracker.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:13
