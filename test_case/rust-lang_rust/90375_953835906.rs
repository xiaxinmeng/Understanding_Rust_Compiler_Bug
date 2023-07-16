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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs at line 1547:
         // Check if a bound would previously have been removed when normalizing
         // the param_env so that it can be given the lowest priority. See
         // #50825 for the motivation for this.
-        let is_global =
-            |cand: &ty::PolyTraitRef<'tcx>| cand.is_global(self.infcx.tcx) && !cand.has_late_bound_regions();
+        let is_global = |cand: &ty::PolyTraitRef<'tcx>| {
+            cand.is_global(self.infcx.tcx) && !cand.has_late_bound_regions()
 
 
         // (*) Prefer `BuiltinCandidate { has_nested: false }`, `PointeeCandidate`,
         // and `DiscriminantKindCandidate` to anything else.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/select/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs" "/checkout/compiler/rustc_traits/src/evaluate_obligation.rs" "/checkout/compiler/rustc_traits/src/type_op.rs" "/checkout/compiler/rustc_traits/src/normalize_projection_ty.rs" "/checkout/compiler/rustc_traits/src/implied_outlives_bounds.rs" "/checkout/compiler/rustc_trait_selection/src/autoderef.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
