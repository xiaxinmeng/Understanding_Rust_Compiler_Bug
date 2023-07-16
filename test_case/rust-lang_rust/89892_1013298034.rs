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
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs at line 621:
                                     let ty = <dyn AstConv<'_>>::ast_ty_to_ty(self, bounded_ty);
                                     match ty.kind() {
                                     match ty.kind() {
-                                        ty::Param(param_ty) if param_ty == expected_ty_as_param => Some(bounds),
-                                        _ => None
+                                        ty::Param(param_ty) if param_ty == expected_ty_as_param => {
+                                            Some(bounds)
+                                        _ => None,
                                     }
-                                },
+                                }
+                                }
                                 _ => None,
                             })
                             .map(|bounds| bounds.iter())
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/inherited.rs" "/checkout/compiler/rustc_typeck/src/check/writeback.rs" "/checkout/compiler/rustc_typeck/src/check/_match.rs" "/checkout/compiler/rustc_typeck/src/check/diverges.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/mod.rs" "/checkout/compiler/rustc_typeck/src/check/autoderef.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs" "/checkout/compiler/rustc_codegen_cranelift/src/optimize/peephole.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
