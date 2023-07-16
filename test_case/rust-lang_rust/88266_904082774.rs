plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
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
Diff in /checkout/compiler/rustc_typeck/src/check/coercion.rs at line 1328:
     /// The inner coercion "engine". If `expression` is `None`, this
     /// is a forced-unit case, and hence `expression_ty` must be
     /// `Nil`.
-    #[instrument(skip(self,fcx,augment_error,label_expression_as_expected), level="debug")]
+    #[instrument(skip(self, fcx, augment_error, label_expression_as_expected), level = "debug")]
     crate fn coerce_inner<'a>(
         &mut self,
         fcx: &FnCtxt<'a, 'tcx>,
Diff in /checkout/compiler/rustc_typeck/src/check/_match.rs at line 14:
 
 
 impl<'a, 'tcx> FnCtxt<'a, 'tcx> {
-    #[instrument(skip(self), level="debug")]
+    #[instrument(skip(self), level = "debug")]
     pub fn check_match(
         &self,
         expr: &'tcx hir::Expr<'tcx>,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/astconv/mod.rs" "/checkout/compiler/rustc_typeck/src/bounds.rs" "/checkout/compiler/rustc_typeck/src/hir_wf_check.rs" "/checkout/compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs" "/checkout/compiler/rustc_typeck/src/constrained_generic_params.rs" "/checkout/compiler/rustc_typeck/src/check/coercion.rs" "/checkout/compiler/rustc_typeck/src/check/intrinsic.rs" "/checkout/compiler/rustc_typeck/src/astconv/errors.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
