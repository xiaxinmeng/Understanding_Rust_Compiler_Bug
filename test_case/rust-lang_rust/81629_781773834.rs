plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0277]: the trait bound `MultiSpan: From<Instance<'_>>` is not satisfied
    --> compiler/rustc_mir/src/borrow_check/diagnostics/conflict_errors.rs:1571:25
     |
1571 |                     err.span_note(instance, "deref defined here");
     |                         ^^^^^^^^^ the trait `From<Instance<'_>>` is not implemented for `MultiSpan`
     = help: the following implementations were found:
     = help: the following implementations were found:
               <MultiSpan as From<Vec<rustc_span::Span>>>
               <MultiSpan as From<rustc_span::Span>>
     = note: required because of the requirements on the impl of `Into<MultiSpan>` for `Instance<'_>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_mir`
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_symbol_mangling" "-p" "rustc_trait_selection" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_typeck" "-p" "rustc_arena" "-p" "rustc_attr" "-p" "rustc_ast_passes" "-p" "rustc_traits" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ty_utils" "-p" "rustc_incremental" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_session" "-p" "rustc_metadata" "-p" "rustc_data_structures" "-p" "rustc_hir" "-p" "rustc_error_codes" "-p" "rustc_errors" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_lint" "-p" "rustc_hir_pretty" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_feature" "-p" "rustc_ast_pretty" "-p" "rustc_span" "-p" "rustc_target" "-p" "rustc_serialize" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_parse" "-p" "rustc_ast" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:50
