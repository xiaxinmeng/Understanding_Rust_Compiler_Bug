plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0282]: type annotations needed
    --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:1352:49
     |
1276 |         let concrete_opaque_types = concrete_opaque_types
     |             --------------------- consider giving `concrete_opaque_types` a type
...
1352 |                         let concrete_ty = match concrete_opaque_types.get(&opaque_type_key) {
     |
     = note: type must be known at this point

error: unused import: `Subst`
error: unused import: `Subst`
  --> compiler/rustc_mir/src/borrow_check/type_check/mod.rs:29:47
   |
29 | use rustc_middle::ty::subst::{GenericArgKind, Subst, SubstsRef, UserSubsts};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.
error: could not compile `rustc_mir`
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_incremental" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_macros" "-p" "rustc_target" "-p" "rustc_apfloat" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_symbol_mangling" "-p" "rustc_session" "-p" "rustc_serialize" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_mir_build" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_hir_pretty" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_lint" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_query_impl" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_privacy" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_save_analysis" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:23
