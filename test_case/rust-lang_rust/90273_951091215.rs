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
Diff in /checkout/compiler/rustc_const_eval/src/transform/check_consts/check.rs at line 934:
                 // const-eval of the `panic_display` fn assumes the argument is `&&str`
                 if Some(callee) == tcx.lang_items().panic_display() {
                     match args[0].ty(&self.ccx.body.local_decls, tcx).kind() {
-                        ty::Ref(_, ty, _) if matches!(ty.kind(), ty::Ref(_, ty, _) if ty.is_str()) => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/transform/promote_consts.rs" "/checkout/compiler/rustc_traits/src/chalk/db.rs" "/checkout/compiler/rustc_const_eval/src/transform/validate.rs" "/checkout/compiler/rustc_traits/src/chalk/lowering.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/post_drop_elaboration.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/resolver.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/mod.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/check.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+                        ty::Ref(_, ty, _) if matches!(ty.kind(), ty::Ref(_, ty, _) if ty.is_str()) =>
                             return;
                         }
                         }
                         _ => self.check_op(ops::PanicNonStr),
