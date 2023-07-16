plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error[E0425]: cannot find value `opaque_typekey` in this scope
    --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:1383:41
     |
1383 | ...                   def_id: opaque_typekey.def_id,
     |                               ^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `opaque_type_key`
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0308]: mismatched types
    --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:1384:41
     |
     |
1384 | ...                   substs: subst_opaque_defn_ty,
     |                               ^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_middle::ty::List`, found struct `TyS`
     |
     = note: expected reference `&rustc_middle::ty::List<rustc_middle::ty::subst::GenericArg<'_>>`
                found reference `&TyS<'_>`
note: return type inferred to be `&rustc_middle::ty::List<rustc_middle::ty::subst::GenericArg<'_>>` here
    --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:1320:25
1320 | /                         infcx
1320 | /                         infcx
1321 | |                             .at(&ObligationCause::dummy(), param_env)
1322 | |                             .eq(output_ty, revealed_ty)?,

error[E0308]: mismatched types
    --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:1387:79
     |
     |
1387 | ...                   let concrete_ty = match concrete_opaque_types.get(opaque_type_key) {
     |                                                                         |
     |                                                                         |
     |                                                                         expected `&OpaqueTypeKey<'_>`, found struct `OpaqueTypeKey`
     |                                                                         help: consider borrowing here: `&opaque_type_key`
     |
note: return type inferred to be `&OpaqueTypeKey<'_>` here
    --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:1320:25
1320 | /                         infcx
1320 | /                         infcx
1321 | |                             .at(&ObligationCause::dummy(), param_env)
1322 | |                             .eq(output_ty, revealed_ty)?,

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0425.
Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_macros" "-p" "rustc_hir_pretty" "-p" "rustc_ast" "-p" "rustc_index" "-p" "rustc_lexer" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_mir" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_graphviz" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "coverage_test_macros" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_serialize" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_target" "-p" "rustc_feature" "-p" "rustc_error_codes" "-p" "rustc_mir_build" "-p" "rustc_interface" "-p" "rustc_ast_passes" "-p" "rustc_resolve" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_expand" "-p" "rustc_privacy" "-p" "rustc_incremental" "-p" "rustc_query_impl" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_ast_pretty" "-p" "rustc_metadata" "-p" "rustc_lint" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:24
