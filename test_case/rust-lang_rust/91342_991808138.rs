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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_const_eval/src/interpret/eval_context.rs at line 347:
     ) -> InterpErrorInfo<'tcx> {
         match err {
             FnAbiError::Layout(err) => err_inval!(Layout(err)).into(),
-            FnAbiError::AdjustForForeignAbi(err) => err_inval!(FnAbiAdjustForForeignAbi(err)).into(),
+            FnAbiError::AdjustForForeignAbi(err) => {
+                err_inval!(FnAbiAdjustForForeignAbi(err)).into()
         }
     }
 }
 }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_builtin_macros/src/concat_bytes.rs" "/checkout/compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs" "/checkout/compiler/rustc_builtin_macros/src/concat.rs" "/checkout/compiler/rustc_builtin_macros/src/panic.rs" "/checkout/compiler/rustc_const_eval/src/interpret/eval_context.rs" "/checkout/compiler/rustc_builtin_macros/src/format.rs" "/checkout/compiler/rustc_const_eval/src/interpret/machine.rs" "/checkout/compiler/rustc_builtin_macros/src/test.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
