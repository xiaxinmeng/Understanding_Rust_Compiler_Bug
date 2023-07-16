plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0621]: explicit lifetime required in the type of `expr`
   --> compiler/rustc_typeck/src/check/demand.rs:681:43
    |
413 |         expr: &hir::Expr<'_>,
    |               -------------- help: add explicit lifetime `'tcx` to the type of `expr`: `&'tcx rustc_hir::Expr<'_>`
...
681 |                                 } else if self.is_else_if_block(expr) {
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime `'tcx` required

error[E0621]: explicit lifetime required in the type of `expr`
   --> compiler/rustc_typeck/src/check/demand.rs:681:43
    |
413 |         expr: &hir::Expr<'_>,
    |               -------------- help: add explicit lifetime `'tcx` to the type of `expr`: `&rustc_hir::Expr<'tcx>`
...
681 |                                 } else if self.is_else_if_block(expr) {
    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^ lifetime `'tcx` required
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0621`.
error: could not compile `rustc_typeck`
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_feature" "-p" "rustc_query_system" "-p" "rustc_fs_util" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_symbol_mangling" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_errors" "-p" "rustc_apfloat" "-p" "rustc_index" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_parse" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_hir_pretty" "-p" "rustc_mir_build" "-p" "rustc_save_analysis" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_query_impl" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_builtin_macros" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_typeck" "-p" "rustc_mir" "-p" "coverage_test_macros" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:53
