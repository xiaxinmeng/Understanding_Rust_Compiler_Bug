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
Diff in /checkout/compiler/rustc_typeck/src/astconv/mod.rs at line 204:
     ) -> ty::Region<'tcx> {
         let tcx = self.tcx();
         if let hir::LifetimeName::ImplicitObjectLifetimeDefault = lifetime.name {
-            panic!("ImplicitObjectLifetimeDefault should not handled separately from ast_region_to_region.");
+            panic!(
+                "ImplicitObjectLifetimeDefault should not handled separately from ast_region_to_region."
         }
 
 
         let r = if let Some(rl) = tcx.named_region(lifetime.hir_id) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/astconv/mod.rs" "/checkout/compiler/rustc_typeck/src/astconv/generics.rs" "/checkout/compiler/rustc_typeck/src/astconv/errors.rs" "/checkout/compiler/rustc_typeck/src/errors.rs" "/checkout/compiler/rustc_typeck/src/check_unused.rs" "/checkout/compiler/rustc_typeck/src/impl_wf_check.rs" "/checkout/compiler/rustc_typeck/src/coherence/inherent_impls.rs" "/checkout/compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
