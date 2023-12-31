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
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 176:
             if supplied_arg_count >= expected_arg_count {
                 (formal_input_tys.to_vec(), expected_input_tys)
             } else {
-                arg_count_error = Some((expected_arg_count, supplied_arg_count, "E0060", false, None));
+                arg_count_error =
+                    Some((expected_arg_count, supplied_arg_count, "E0060", false, None));
                 (self.err_args(supplied_arg_count), vec![])
         } else {
         } else {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/mod.rs" "/checkout/compiler/rustc_typeck/src/check/compare_method.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs" "/checkout/compiler/rustc_typeck/src/check/coercion.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs" "/checkout/compiler/rustc_typeck/src/check/writeback.rs" "/checkout/compiler/rustc_typeck/src/check/expectation.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
