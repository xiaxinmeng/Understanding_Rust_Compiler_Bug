plain
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_hir v0.0.0 (/checkout/compiler/rustc_hir)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking chalk-engine v0.36.0
error[E0599]: no method named `features` found for reference `&Session` in the current scope
   --> compiler/rustc_hir/src/hir.rs:299:60
    |
299 |                 ast::ParamKindOrd::Const { unordered: sess.features().const_generics }
    |                                                            ^^^^^^^^ private field, not a method
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_hir`
error: could not compile `rustc_hir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_incremental" "-p" "rustc_apfloat" "-p" "rustc_hir" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_errors" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_hir_pretty" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_mir_build" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_privacy" "-p" "rustc_typeck" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:50
