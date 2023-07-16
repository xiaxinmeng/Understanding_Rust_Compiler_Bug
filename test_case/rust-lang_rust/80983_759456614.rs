plain
    Checking rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
    Checking rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: unused import: `rustc_session::utils::NativeLibKind`
   |
   |
32 | use rustc_session::utils::NativeLibKind;
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: aborting due to previous error

error: could not compile `rustc_codegen_ssa`
error: could not compile `rustc_codegen_ssa`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_lint_defs" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_fs_util" "-p" "rustc_macros" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_incremental" "-p" "rustc_errors" "-p" "rustc_serialize" "-p" "rustc_hir" "-p" "rustc_symbol_mangling" "-p" "rustc_target" "-p" "rustc_apfloat" "-p" "rustc_ast" "-p" "rustc_index" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_interface" "-p" "rustc_typeck" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_ast_passes" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_mir_build" "-p" "rustc_builtin_macros" "-p" "rustc_expand" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_save_analysis" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:40
