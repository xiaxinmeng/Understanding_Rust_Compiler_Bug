plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0107]: this struct takes 1 lifetime argument but 2 lifetime arguments were supplied
     |
     |
168  |     rustc_data_structures::sync::assert_sync::<tls::ImplicitCtxt<'_, '_>>();
     |                                                     ^^^^^^^^^^^^   ---- help: remove this lifetime argument
     |                                                     expected 1 lifetime argument
     |
     |
note: struct defined here, with 1 lifetime parameter: `'tcx`
    --> /checkout/compiler/rustc_middle/src/ty/context.rs:1642:16
1642 |     pub struct ImplicitCtxt<'tcx> {
     |                ^^^^^^^^^^^^ ----

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
error: could not compile `rustc_interface`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_macros" "-p" "rustc_serialize" "-p" "rustc_hir_pretty" "-p" "rustc_metadata" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_feature" "-p" "rustc_ast_pretty" "-p" "rustc_target" "-p" "rustc_mir" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_graphviz" "-p" "rustc_infer" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_plugin_impl" "-p" "rustc_hir" "-p" "rustc_save_analysis" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_mir_build" "-p" "rustc_span" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_parse" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_symbol_mangling" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_incremental" "-p" "rustc_query_impl" "-p" "rustc_data_structures" "-p" "rustc_error_codes" "-p" "rustc_ast" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:28
