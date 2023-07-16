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
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 123:
         projection: ty::ProjectionTy<'tcx>,
     ) -> ControlFlow<V::BreakTy> {
         let tcx = self.def_id_visitor.tcx();
-        let (trait_ref, assoc_substs) = if tcx.def_path(projection.item_def_id).get_impl_trait_in_trait_data().is_none()
+        let (trait_ref, assoc_substs) = if tcx
+            .def_path(projection.item_def_id)
+            .is_none()
         {
         {
             projection.trait_ref_and_own_substs(tcx)
         } else {
Diff in /checkout/compiler/rustc_privacy/src/lib.rs at line 130:
             // HACK(RPITIT): Remove this when RPITITs are lowered to regular assoc tys
+            let (def_id, _) =
+            let (def_id, _) =
                 tcx.def_path(projection.item_def_id).get_impl_trait_in_trait_data().unwrap();
-           let trait_generics = tcx.generics_of(def_id);
-           (
-               ty::TraitRef { def_id, substs: projection.substs.truncate_to(tcx, trait_generics) },
-               &projection.substs[trait_generics.count()..],
+            let trait_generics = tcx.generics_of(def_id);
+            (
+            (
+                ty::TraitRef { def_id, substs: projection.substs.truncate_to(tcx, trait_generics) },
+                &projection.substs[trait_generics.count()..],
         };
         };
         self.visit_trait(trait_ref)?;
         if self.def_id_visitor.shallow() {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_baked_icu_data/src/data/fallback/supplement/mod.rs" "/checkout/compiler/rustc_baked_icu_data/src/data/fallback/likelysubtags_v1.rs" "/checkout/compiler/rustc_baked_icu_data/src/data/fallback/mod.rs" "/checkout/compiler/rustc_privacy/src/errors.rs" "/checkout/compiler/rustc_privacy/src/lib.rs" "/checkout/compiler/rustc_ast/src/node_id.rs" "/checkout/compiler/rustc_ast/src/token.rs" "/checkout/compiler/rustc_baked_icu_data/src/data/fallback/parents_v1.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
