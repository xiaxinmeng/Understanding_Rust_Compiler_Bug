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
Diff in /checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs at line 1487:
                     let mut returned_async_output_error = false;
                     for sp in values {
                         if sp.is_desugaring(DesugaringKind::Async) && !returned_async_output_error {
-                            err.span_label(
-                                *sp,
-                                format!("{}", "async functions return futures"),
-                            );
+                            err.span_label(*sp, format!("{}", "async functions return futures"));
                             err.span_label(
                                 *sp,
                                 *sp,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_lexer/src/unescape.rs" "/checkout/compiler/rustc_ast_lowering/src/expr.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/mod.rs" "/checkout/compiler/rustc_infer/src/infer/at.rs" "/checkout/compiler/rustc_infer/src/infer/sub.rs" "/checkout/compiler/rustc_infer/src/infer/error_reporting/note.rs" "/checkout/compiler/rustc_infer/src/infer/canonical/mod.rs" "/checkout/compiler/rustc_codegen_ssa/src/traits/abi.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
