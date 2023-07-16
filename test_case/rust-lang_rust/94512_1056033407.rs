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
Diff in /checkout/compiler/rustc_const_eval/src/interpret/operator.rs at line 203:
                 // on overflow, which can happen with "int_min $OP -1".
                 if matches!(bin_op, Rem | Div) {
                     if l == size.signed_int_min() && r == -1 {
-                        if bin_op == Rem { throw_ub!(RemainderOverflow) } else { throw_ub!(DivisionOverflow) }
+                        if bin_op == Rem {
+                            throw_ub!(RemainderOverflow)
+                        } else {
+                            throw_ub!(DivisionOverflow)
                     }
                 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/validity.rs" "/checkout/compiler/rustc_const_eval/src/interpret/step.rs" "/checkout/compiler/rustc_const_eval/src/util/alignment.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operator.rs" "/checkout/compiler/rustc_const_eval/src/util/find_self_call.rs" "/checkout/compiler/rustc_const_eval/src/interpret/traits.rs" "/checkout/compiler/rustc_const_eval/src/util/call_kind.rs" "/checkout/compiler/rustc_const_eval/src/interpret/terminator.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
