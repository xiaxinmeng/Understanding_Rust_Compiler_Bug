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
Diff in /checkout/compiler/rustc_middle/src/ty/layout.rs at line 2581:
     // for `Instance` (e.g. typeck would use `Ty::fn_sig` instead),
     // or should go through `FnAbi` instead, to avoid losing any
     // adjustments `fn_abi_of_instance` might be performing.
-    fn fn_sig_for_fn_abi(&self, tcx: TyCtxt<'tcx>, param_env: ty::ParamEnv<'tcx>) -> ty::PolyFnSig<'tcx> {
+    fn fn_sig_for_fn_abi(
+        &self,
+        tcx: TyCtxt<'tcx>,
+        param_env: ty::ParamEnv<'tcx>,
+    ) -> ty::PolyFnSig<'tcx> {
         let ty = self.ty(tcx, param_env);
         match *ty.kind() {
             ty::FnDef(..) => {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/ty/layout.rs" "/checkout/compiler/rustc_middle/src/ty/structural_impls.rs" "/checkout/compiler/rustc_middle/src/ty/sty.rs" "/checkout/compiler/rustc_middle/src/ty/trait_def.rs" "/checkout/compiler/rustc_middle/src/ty/fast_reject.rs" "/checkout/compiler/rustc_middle/src/ty/query.rs" "/checkout/compiler/rustc_middle/src/ty/vtable.rs" "/checkout/compiler/rustc_middle/src/ty/adjustment.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
