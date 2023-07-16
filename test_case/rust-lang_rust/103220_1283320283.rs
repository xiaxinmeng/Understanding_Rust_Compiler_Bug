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
Diff in /checkout/compiler/rustc_trait_selection/src/traits/mod.rs at line 764:
     });
 }
 
-fn own_existential_vtable_entries<'tcx>(
-    tcx: TyCtxt<'tcx>,
-    trait_def_id: DefId,
-) -> &'tcx [DefId] {
+fn own_existential_vtable_entries<'tcx>(tcx: TyCtxt<'tcx>, trait_def_id: DefId) -> &'tcx [DefId] {
     let trait_methods = tcx
         .associated_items(trait_def_id)
         .in_definition_order()
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_trait_selection/src/traits/select/confirmation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/normalize.rs" "/checkout/compiler/rustc_trait_selection/src/traits/coherence.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/type_op/normalize.rs" "/checkout/compiler/rustc_trait_selection/src/traits/query/mod.rs" "/checkout/compiler/rustc_trait_selection/src/traits/chalk_fulfill.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
