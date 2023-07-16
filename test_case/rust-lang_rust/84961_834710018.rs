plain
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error[E0277]: expected a `FnOnce<()>` closure, found `F`
    |
    |
104 | pub fn create_session_globals_then<R>(edition: Edition, f: impl FnOnce() -> R) -> R {
    |                                                                 ------------- required by this bound in `create_session_globals_then`
...
157 |     create_session_globals_then(edition, f)
    |                                          ^ expected an `FnOnce<()>` closure, found `F`
    |
    = note: wrap the `F` in a closure with no arguments: `|| { /* code */ }`
help: consider further restricting this bound
    |
155 |     F: FnOnce(&SessionGlobals) -> R + FnOnce<()>,

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_span`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_hir_pretty" "-p" "rustc_feature" "-p" "rustc_metadata" "-p" "rustc_attr" "-p" "rustc_index" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_macros" "-p" "rustc_ast_pretty" "-p" "rustc_typeck" "-p" "rustc_arena" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_error_codes" "-p" "rustc_ast" "-p" "rustc_span" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_errors" "-p" "rustc_save_analysis" "-p" "rustc_mir_build" "-p" "rustc_plugin_impl" "-p" "rustc_serialize" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_session" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:50
