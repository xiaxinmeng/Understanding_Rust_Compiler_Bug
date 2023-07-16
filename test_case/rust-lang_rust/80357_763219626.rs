plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error[E0433]: failed to resolve: use of undeclared type `ExprKind`
 --> compiler/rustc_mir_build/src/build/expr/looking_for_a_better_name.rs:5:9
  |
5 |         ExprKind::If { .. } => {
  |         ^^^^^^^^ use of undeclared type `ExprKind`

error[E0433]: failed to resolve: use of undeclared type `ExprKind`
 --> compiler/rustc_mir_build/src/build/expr/looking_for_a_better_name.rs:8:9
  |
8 |         ExprKind::Let { .. } => {
  |         ^^^^^^^^ use of undeclared type `ExprKind`

error[E0433]: failed to resolve: use of undeclared type `ExprKind`
  --> compiler/rustc_mir_build/src/build/expr/looking_for_a_better_name.rs:11:9
   |
11 |         ExprKind::Scope { .. } => {
   |         ^^^^^^^^ use of undeclared type `ExprKind`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_mir_build`
error: could not compile `rustc_mir_build`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_apfloat" "-p" "rustc_symbol_mangling" "-p" "rustc_serialize" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_hir" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_data_structures" "-p" "rustc_macros" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_span" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_fs_util" "-p" "rustc_index" "-p" "rustc_ast" "-p" "rustc_target" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_interface" "-p" "rustc_typeck" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_mir_build" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:57
