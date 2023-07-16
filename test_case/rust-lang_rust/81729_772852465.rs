plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error: no rules expected the token `,`
   |
   |
29 |     "detect braces in single-argument panic!() invocations",
   |                                                            ^ no rules expected this token in macro call

error[E0425]: cannot find value `NON_FMT_PANIC` in this scope
   |
   |
33 | declare_lint_pass!(PanicFmt => [NON_FMT_PANIC]);


error[E0425]: cannot find value `NON_FMT_PANIC` in this scope
    |
    |
102 |                         cx.struct_span_lint(NON_FMT_PANIC, arg_spans, |lint| {


error[E0425]: cannot find value `NON_FMT_PANIC` in this scope
    |
    |
139 |                         cx.struct_span_lint(NON_FMT_PANIC, brace_spans.unwrap_or(vec![expn.call_site]), |lint| {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0425`.
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_lint`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_ast_pretty" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_mir" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_attr" "-p" "rustc_infer" "-p" "rustc_apfloat" "-p" "coverage_test_macros" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ast_passes" "-p" "rustc_lint" "-p" "rustc_hir_pretty" "-p" "rustc_ast" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_errors" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_serialize" "-p" "rustc_plugin_impl" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_span" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_incremental" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_typeck" "-p" "rustc_mir_build" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:26
