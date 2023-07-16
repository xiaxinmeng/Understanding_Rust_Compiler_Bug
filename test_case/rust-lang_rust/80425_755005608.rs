plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0560]: struct `ModuleData<'_>` has no field named `closest_parent_mod`
    |
518 |             closest_parent_mod,
518 |             closest_parent_mod,
    |             ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&ModuleData<'_>`
     |
     |
2135 |         let module = self.get_module(DefId { index: CRATE_DEF_INDEX, ..module.closest_parent_mod });
     |                                                                               ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&'a ModuleData<'a>`
     |
     |
2147 |         let mut module = self.get_module(module.closest_parent_mod);
     |                                                 ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&ModuleData<'_>`
     |
     |
2150 |             module = self.get_module(parent.closest_parent_mod);
     |                                             ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&'a ModuleData<'a>`
     |
     |
2812 |         vis.is_accessible_from(module.closest_parent_mod, self)
     |                                       ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&&ModuleData<'a>`
     |
     |
2836 |                 macro_rules.closest_parent_mod == modularized.closest_parent_mod
     |                             ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&&ModuleData<'a>`
     |
     |
2836 |                 macro_rules.closest_parent_mod == modularized.closest_parent_mod
     |                                                               ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&'a ModuleData<'a>`
   --> compiler/rustc_resolve/src/build_reduced_graph.rs:269:76
    |
269 |                     Ok(ty::Visibility::Restricted(self.parent_scope.module.closest_parent_mod))
    |                                                                            ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&ModuleData<'a>`
   --> compiler/rustc_resolve/src/build_reduced_graph.rs:806:28
806 |                     parent.closest_parent_mod,
806 |                     parent.closest_parent_mod,
    |                            ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&ModuleData<'a>`
   --> compiler/rustc_resolve/src/build_reduced_graph.rs:881:28
881 |                     parent.closest_parent_mod,
881 |                     parent.closest_parent_mod,
    |                            ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&ModuleData<'a>`
   --> compiler/rustc_resolve/src/build_reduced_graph.rs:924:24
924 |                 parent.closest_parent_mod,
924 |                 parent.closest_parent_mod,
    |                        ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&'a ModuleData<'a>`
    --> compiler/rustc_resolve/src/late.rs:1778:55
     |
1778 |                 let def_id = this.parent_scope.module.closest_parent_mod;
     |                                                       ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&'a ModuleData<'a>`
    --> compiler/rustc_resolve/src/late.rs:1846:51
     |
1846 |             let def_id = this.parent_scope.module.closest_parent_mod;
     |                                                   ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`

error[E0609]: no field `closest_parent_mod` on type `&ModuleData<'_>`
    |
    |
331 |             let normal_module_def_id = self.macro_def_scope(invoc_id).closest_parent_mod;
    |                                                                       ^^^^^^^^^^^^^^^^^^ help: a field with a similar name exists: `nearest_parent_mod`
error: aborting due to 14 previous errors

Some errors have detailed explanations: E0560, E0609.
For more information about an error, try `rustc --explain E0560`.
For more information about an error, try `rustc --explain E0560`.
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: could not compile `rustc_resolve`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_mir" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "coverage_test_macros" "-p" "rustc_macros" "-p" "rustc_apfloat" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_ast" "-p" "rustc_lint" "-p" "rustc_save_analysis" "-p" "rustc_target" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_typeck" "-p" "rustc_arena" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_mir_build" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_lowering" "-p" "rustc_incremental" "-p" "rustc_traits" "-p" "rustc_builtin_macros" "-p" "rustc_error_codes" "-p" "rustc_hir" "-p" "rustc_ast_pretty" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_data_structures" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_hir_pretty" "-p" "rustc_feature" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:15
