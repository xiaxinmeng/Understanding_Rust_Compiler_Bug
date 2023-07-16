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
Diff in /checkout/compiler/rustc_const_eval/src/transform/check_consts/check.rs at line 927:
 
                         // Calling an unstable function *always* requires that the corresponding gate
                         // be enabled, even if the function has `#[rustc_allow_const_fn_unstable(the_gate)]`.
-                        if !tcx.features().declared_lib_features.iter().any(|&(sym, _)| sym == feature)
+                        if !tcx
+                            .features()
+                            .declared_lib_features
+                            .iter()
+                            .any(|&(sym, _)| sym == feature)
                         {
                             bad_gates.push(feature);
                             continue;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/eval_context.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics/caller_location.rs" "/checkout/compiler/rustc_const_eval/src/interpret/visitor.rs" "/checkout/compiler/rustc_const_eval/src/interpret/terminator.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/check.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intern.rs" "/checkout/compiler/rustc_const_eval/src/transform/check_consts/ops.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics/type_name.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
