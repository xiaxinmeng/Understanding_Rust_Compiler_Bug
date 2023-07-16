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
Diff in /checkout/compiler/rustc_const_eval/src/interpret/eval_context.rs at line 215:
         match self.value {
             LocalValue::Dead => throw_ub!(DeadLocal),
             LocalValue::Live(Operand::Indirect(mplace)) => Ok(Err(mplace)),
-            ref mut
-            local @ (LocalValue::Live(Operand::Immediate(_)) | LocalValue::Unallocated) => {
+            ref mut local @ (LocalValue::Live(Operand::Immediate(_)) | LocalValue::Unallocated) => {
                 Ok(Ok(local))
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/util/find_self_call.rs" "/checkout/compiler/rustc_const_eval/src/interpret/place.rs" "/checkout/compiler/rustc_const_eval/src/util/call_kind.rs" "/checkout/compiler/rustc_const_eval/src/interpret/validity.rs" "/checkout/compiler/rustc_const_eval/src/interpret/step.rs" "/checkout/compiler/rustc_const_eval/src/interpret/eval_context.rs" "/checkout/compiler/rustc_const_eval/src/interpret/visitor.rs" "/checkout/compiler/rustc_const_eval/src/interpret/operand.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
