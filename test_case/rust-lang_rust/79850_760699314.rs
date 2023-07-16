plain
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error[E0425]: cannot find value `path_def_id` in this scope
   --> compiler/rustc_passes/src/liveness.rs:353:59
    |
353 | ...                   if begin_panic_def_id == Some(path_def_id) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_passes`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_hir" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_mir" "-p" "rustc_infer" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_graphviz" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_builtin_macros" "-p" "rustc_expand" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_privacy" "-p" "rustc_ast_lowering" "-p" "rustc_arena" "-p" "rustc_passes" "-p" "rustc_typeck" "-p" "rustc_resolve" "-p" "rustc_ast_passes" "-p" "rustc_mir_build" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_span" "-p" "rustc_feature" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_serialize" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_metadata" "-p" "rustc_ast" "-p" "rustc_save_analysis" "-p" "rustc_hir_pretty" "-p" "rustc_errors" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_data_structures" "-p" "rustc_error_codes" "-p" "rustc_target" "-p" "rustc_ast_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:57
