plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking tracing-tree v0.1.9
error[E0560]: struct `FutureIncompatibleInfo` has no field named `edition`
    --> compiler/rustc_lint_defs/src/builtin.rs:3299:9
     |
3299 |         edition: Some(Edition::Edition2021),
     |         ^^^^^^^ `FutureIncompatibleInfo` does not have this field
     |
     = note: available fields are: `reference`, `reason`, `future_breakage`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0560`.
error: could not compile `rustc_lint_defs`
error: could not compile `rustc_lint_defs`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_target" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_feature" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint_defs" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_attr" "-p" "rustc_parse_format" "-p" "rustc_errors" "-p" "rustc_mir" "-p" "rustc_apfloat" "-p" "coverage_test_macros" "-p" "rustc_ast" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_save_analysis" "-p" "rustc_data_structures" "-p" "rustc_serialize" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_ast_pretty" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_incremental" "-p" "rustc_query_impl" "-p" "rustc_traits" "-p" "rustc_mir_build" "-p" "rustc_hir" "-p" "rustc_error_codes" "-p" "rustc_typeck" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:57
