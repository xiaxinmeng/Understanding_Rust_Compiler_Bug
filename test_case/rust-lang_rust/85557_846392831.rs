plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: unused variable: `ecx`
   --> compiler/rustc_mir/src/interpret/machine.rs:136:20
    |
136 |     fn enforce_abi(ecx: &InterpCx<'mir, 'tcx, Self>) -> bool {
    |                    ^^^ help: if this is intentional, prefix it with an underscore: `_ecx`
    |
    = note: `-D unused-variables` implied by `-D warnings`
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error: aborting due to previous error

error: could not compile `rustc_mir`
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_query_impl" "-p" "rustc_macros" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_passes" "-p" "rustc_lexer" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_fs_util" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_ast_passes" "-p" "rustc_index" "-p" "rustc_resolve" "-p" "rustc_attr" "-p" "rustc_traits" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ast_lowering" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_plugin_impl" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_target" "-p" "rustc_lint" "-p" "rustc_parse" "-p" "rustc_mir_build" "-p" "rustc_metadata" "-p" "rustc_error_codes" "-p" "rustc_ast_pretty" "-p" "rustc_errors" "-p" "rustc_typeck" "-p" "rustc_session" "-p" "rustc_ast" "-p" "rustc_hir_pretty" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_save_analysis" "-p" "rustc_data_structures" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:29
