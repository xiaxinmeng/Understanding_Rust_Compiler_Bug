plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_hir_typeck/src/fn_ctxt/suggestions.rs at line 925:
             let ty = match self.tcx.asyncness(fn_id.owner) {
                 hir::IsAsync::Async => {
                     let infcx = self.tcx.infer_ctxt().build();
-                    infcx
-                        .get_impl_future_output_ty(ty)
-                        .unwrap_or_else(|| {
-                            span_bug!(
-                                fn_decl.output.span(),
-                                "failed to get output type of async function"
-                        })
-                        })
+                    infcx.get_impl_future_output_ty(ty).unwrap_or_else(|| {
+                        span_bug!(
+                            fn_decl.output.span(),
+                            "failed to get output type of async function"
+                    })
                 }
                 }
                 hir::IsAsync::NotAsync => ty,
             };
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/mod.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/suggestions.rs" "/checkout/compiler/rustc_hir_typeck/src/place_op.rs" "/checkout/compiler/rustc_session/src/parse.rs" "/checkout/compiler/rustc_hir_typeck/src/coercion.rs" "/checkout/compiler/rustc_hir_typeck/src/fn_ctxt/arg_matrix.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
