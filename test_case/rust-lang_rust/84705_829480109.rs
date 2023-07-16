plain
    |                         ----^^^
    |                         |
    |                         help: remove this `mut`
    |
    = note: `-D unused-mut` implied by `-D warnings`

error[E0716]: temporary value dropped while borrowed
   --> compiler/rustc_typeck/src/check/wfcheck.rs:318:35
318 |                       let mut err = tcx
    |  ___________________________________^
319 | |                         .sess
320 | |                         .struct_span_err(
320 | |                         .struct_span_err(
321 | |                             hir_ty.span,
325 | |                             ),
326 | |                         )
326 | |                         )
    | |_________________________^ creates a temporary which is freed while still in use
327 |                           .note("the only supported types are integers, `bool` and `char`");
    |                                                                                            - temporary value is freed at the end of this statement
328 |                       if tcx.sess.is_nightly_build() {
329 |                           err.help(
    |                           --- borrow later used here
    |
    = note: consider using a `let` binding to create a longer lived value
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
error: could not compile `rustc_typeck`
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_middle" "-p" "rustc_apfloat" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_type_ir" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_typeck" "-p" "rustc_infer" "-p" "rustc_ty_utils" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_feature" "-p" "rustc_plugin_impl" "-p" "rustc_target" "-p" "rustc_error_codes" "-p" "rustc_span" "-p" "rustc_interface" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_ast_passes" "-p" "rustc_query_impl" "-p" "rustc_symbol_mangling" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_incremental" "-p" "rustc_serialize" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_lint" "-p" "rustc_mir_build" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_errors" "-p" "rustc_metadata" "-p" "rustc_hir_pretty" "-p" "rustc_ast" "-p" "rustc_session" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:58
