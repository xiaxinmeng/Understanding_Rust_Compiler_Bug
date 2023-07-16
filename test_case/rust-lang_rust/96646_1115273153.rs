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
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs at line 702:
 
 
                         if let Some(expr) = provided_args.get(input_idx) {
-                        self.emit_coerce_suggestions(
-                            &mut err,
+                            self.emit_coerce_suggestions(
                                 &expr,
                                 &expr,
-                            final_arg_types[input_idx].map(|ty| ty.0).unwrap(),
-                            final_arg_types[input_idx].map(|ty| ty.1).unwrap(),
-                            None,
-                        );
-                        );
+                                final_arg_types[input_idx].map(|ty| ty.0).unwrap(),
+                                final_arg_types[input_idx].map(|ty| ty.1).unwrap(),
+                                None,
+                            );
                         }
                     }
                     }
                     Error::Extra(arg_idx) => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/closure.rs" "/checkout/compiler/rustc_typeck/src/check/expr.rs" "/checkout/compiler/rustc_typeck/src/check/callee.rs" "/checkout/compiler/rustc_typeck/src/check/mod.rs" "/checkout/compiler/rustc_typeck/src/check/pat.rs" "/checkout/compiler/rustc_typeck/src/check/autoderef.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs" "/checkout/compiler/rustc_codegen_llvm/src/llvm/archive_ro.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
