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
Diff in /checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs at line 609:
             let ty = self.tcx.erase_late_bound_regions(Binder::bind_with_vars(ty, bound_vars));
             let ty = self.normalize_associated_types_in(expr.span, ty);
             let ty = match self.tcx.asyncness(fn_id.owner) {
-                hir::IsAsync::Async => self.tcx.infer_ctxt().enter(|infcx| {
-                    infcx.get_impl_future_output_ty(ty).unwrap_or_else(|| {
-                        span_bug!(
-                            fn_decl.output.span(),
-                            "failed to get output type of async function"
-                        )
+                hir::IsAsync::Async => self
+                    .tcx
+                    .infer_ctxt()
+                    .enter(|infcx| {
+                        infcx.get_impl_future_output_ty(ty).unwrap_or_else(|| {
+                            span_bug!(
+                                fn_decl.output.span(),
+                                "failed to get output type of async function"
+                        })
                     })
-                })
-                .skip_binder(),
-                .skip_binder(),
+                    .skip_binder(),
                 hir::IsAsync::NotAsync => ty,
             };
             if self.can_coerce(found, ty) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/suggestions.rs" "/checkout/compiler/rustc_typeck/src/check/op.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/check/inherited.rs" "/checkout/compiler/rustc_typeck/src/check/fn_ctxt/checks.rs" "/checkout/compiler/rustc_typeck/src/check/writeback.rs" "/checkout/compiler/rustc_typeck/src/check/regionck.rs"` failed.
Build completed unsuccessfully in 0:00:13
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
