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
Diff in /checkout/compiler/rustc_const_eval/src/interpret/operator.rs at line 330:
             _ if left.layout.ty.is_any_ptr() => {
                 // The RHS type must be the same *or an integer type* (for `Offset`).
                 assert!(
-                    right.layout.ty.is_any_ptr()|| right.layout.ty.is_integral(),
+                    right.layout.ty.is_any_ptr() || right.layout.ty.is_integral(),
                     "Unexpected types for BinOp: {:?} {:?} {:?}",
                     left.layout.ty,
                     bin_op,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/place.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/eval_queries.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intern.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/error.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operator.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/mod.rs" "/checkout/compiler/rustc_const_eval/src/interpret/eval_context.rs" "/checkout/compiler/rustc_const_eval/src/const_eval/machine.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
