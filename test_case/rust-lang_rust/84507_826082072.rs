plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: unused variable: `def_id`
   --> compiler/rustc_passes/src/entry.rs:167:25
    |
167 |     } else if let Some((def_id, DefKind::Fn)) = tcx.hir().krate().main_by_path {
    |                         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_def_id`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_passes`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_error_codes" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_feature" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_interface" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_symbol_mangling" "-p" "rustc_passes" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_traits" "-p" "rustc_ast_passes" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_trait_selection" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_ast_lowering" "-p" "rustc_ast_pretty" "-p" "rustc_metadata" "-p" "rustc_typeck" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_apfloat" "-p" "rustc_lint" "-p" "rustc_hir_pretty" "-p" "rustc_session" "-p" "rustc_hir" "-p" "rustc_save_analysis" "-p" "rustc_errors" "-p" "rustc_mir_build" "-p" "rustc_ast" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_target" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:04
