plain
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0425]: cannot find value `bool_not_method` in module `sym`
   --> compiler/rustc_feature/src/active.rs:648:14
    |
648 |     (active, bool_not_method, "1.52.0", None, None),
    |              ^^^^^^^^^^^^^^^ not found in `sym`

error[E0531]: cannot find unit struct, unit variant or constant `bool_not_method` in module `sym`
   --> compiler/rustc_feature/src/active.rs:648:14
    |
648 |     (active, bool_not_method, "1.52.0", None, None),
    |              ^^^^^^^^^^^^^^^ not found in `sym`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0531.
For more information about an error, try `rustc --explain E0425`.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustc_feature`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_feature" "-p" "rustc_mir" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_attr" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "coverage_test_macros" "-p" "rustc_lexer" "-p" "rustc_apfloat" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_serialize" "-p" "rustc_data_structures" "-p" "rustc_plugin_impl" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_error_codes" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_lint" "-p" "rustc_hir_pretty" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_ast" "-p" "rustc_errors" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_query_impl" "-p" "rustc_resolve" "-p" "rustc_mir_build" "-p" "rustc_ast_lowering" "-p" "rustc_incremental" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_typeck" "-p" "rustc_builtin_macros" "-p" "rustc_ty_utils" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:01
