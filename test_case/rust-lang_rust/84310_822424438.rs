plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0425]: cannot find value `ccx` in this scope
   --> compiler/rustc_mir/src/transform/check_consts/validation.rs:237:21
    |
237 |                     ccx.tcx
    |                     ^^^ help: you might have meant to use the available field: `self.ccx`
error[E0423]: expected value, found macro `span`
   --> compiler/rustc_mir/src/transform/check_consts/validation.rs:239:42
    |
    |
239 |                         .struct_span_err(span, "trait methods cannot be stable const fn")
    |                                          ^^^^ help: you might have meant to use the available field: `self.span`
error: unused import: `struct_span_err`
 --> compiler/rustc_mir/src/transform/check_consts/validation.rs:3:20
  |
  |
3 | use rustc_errors::{struct_span_err, Applicability, Diagnostic, ErrorReported};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0423, E0425.
Some errors have detailed explanations: E0423, E0425.
For more information about an error, try `rustc --explain E0423`.
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_macros" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_symbol_mangling" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_index" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_data_structures" "-p" "rustc_session" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_target" "-p" "rustc_serialize" "-p" "rustc_fs_util" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_mir_build" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_expand" "-p" "rustc_passes" "-p" "rustc_query_impl" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_builtin_macros" "-p" "rustc_ty_utils" "-p" "rustc_ast_passes" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_metadata" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "rustc_hir_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_typeck" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:51
