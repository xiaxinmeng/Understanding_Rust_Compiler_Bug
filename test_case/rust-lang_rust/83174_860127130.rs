plain
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
   --> compiler/rustc_mir/src/borrow_check/diagnostics/conflict_errors.rs:724:13
    |
724 |             BorrowExplanation::UsedLater(LaterUseKind::Call, _call_span) => {
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
   ::: compiler/rustc_mir/src/borrow_check/diagnostics/explain_borrow.rs:28:5
    |
    |
28  |     UsedLater(LaterUseKind, Span, Option<Span>),
    |     ------------------------------------------- tuple variant defined here
    |
help: use `_` to explicitly ignore each field
    |
724 |             BorrowExplanation::UsedLater(LaterUseKind::Call, _call_span, _) => {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0023`.
For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_target" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_hir" "-p" "rustc_interface" "-p" "rustc_symbol_mangling" "-p" "rustc_expand" "-p" "rustc_lexer" "-p" "rustc_lint_defs" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_fs_util" "-p" "rustc_ast_passes" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_attr" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_infer" "-p" "rustc_resolve" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_span" "-p" "rustc_hir_pretty" "-p" "rustc_session" "-p" "rustc_serialize" "-p" "rustc_ast" "-p" "rustc_lint" "-p" "rustc_save_analysis" "-p" "rustc_errors" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_metadata" "-p" "rustc_data_structures" "-p" "rustc_ast_pretty" "-p" "rustc_feature" "-p" "rustc_error_codes" "-p" "rustc_typeck" "-p" "rustc_plugin_impl" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_parse" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:59
