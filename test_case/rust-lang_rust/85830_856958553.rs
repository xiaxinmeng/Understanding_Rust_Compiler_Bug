plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0308]: mismatched types
   --> compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:158:5
    |
158 |     is_private_dep => { cdata.private_dep }
    |     ^^^^^^^^^^^^^^ expected `()`, found fn item
    = note: expected unit type `()`
    = note: expected unit type `()`
                 found fn item `for<'tcx> fn(TyCtxt<'tcx>, CrateNum) -> bool {provide_extern::is_private_dep}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_metadata`
error: could not compile `rustc_metadata`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_feature" "-p" "rustc_span" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_apfloat" "-p" "rustc_target" "-p" "rustc_symbol_mangling" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_macros" "-p" "rustc_errors" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_data_structures" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_query_impl" "-p" "rustc_hir_pretty" "-p" "rustc_mir_build" "-p" "rustc_plugin_impl" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:13
