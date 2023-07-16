plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0308]: mismatched types
    --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:1389:79
     |
1389 | ...                   let concrete_ty = match concrete_opaque_types.get(opaque_type_key) {
     |                                                                         ^^^^^^^^^^^^^^^ expected `&OpaqueTypeKey<'_>`, found `()`
     |
note: return type inferred to be `&OpaqueTypeKey<'_>` here
    --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:1320:25
1320 | /                         infcx
1320 | /                         infcx
1321 | |                             .at(&ObligationCause::dummy(), param_env)
1322 | |                             .eq(output_ty, revealed_ty)?,

error[E0609]: no field `def_id` on type `()`
    --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:1396:65
     |
     |
1396 | ...                   opaque_type_key.def_id,
     |                                       ^^^^^^

error[E0271]: type mismatch resolving `<VecMap<(), &TyS<'_>> as IntoIterator>::Item == (OpaqueTypeKey<'tcx>, &'tcx TyS<'tcx>)`
    --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:1443:33
     |
1443 |         self.opaque_type_values.extend(opaque_type_values);
     |                                 ^^^^^^ expected `()`, found struct `OpaqueTypeKey`
     |
     = note: expected tuple `((), &TyS<'_>)`
                found tuple `(OpaqueTypeKey<'tcx>, &'tcx TyS<'tcx>)`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0271, E0308, E0609.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_lint_defs" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_symbol_mangling" "-p" "rustc_errors" "-p" "rustc_hir" "-p" "rustc_serialize" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_incremental" "-p" "rustc_apfloat" "-p" "rustc_fs_util" "-p" "rustc_ast" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_query_impl" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_typeck" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_hir_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_mir_build" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:11
