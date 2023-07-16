plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error[E0593]: function is expected to take 1 argument, but it takes 0 arguments
    --> compiler/rustc_typeck/src/check/mod.rs:1068:26
     |
1068 |             .map_or_else(String::new, |s| format!(" `{}`", s)),
     |                          ^^^^^^^^^^^ expected function that takes 1 argument
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0593]: function is expected to take 1 argument, but it takes 0 arguments
   --> compiler/rustc_typeck/src/check/pat.rs:882:30
    |
    |
882 |                 .map_or_else(String::new, |s| format!(" `{}`", s.trim_end()));
    |                              ^^^^^^^^^^^ expected function that takes 1 argument
error[E0593]: function is expected to take 1 argument, but it takes 0 arguments
    --> compiler/rustc_typeck/src/collect.rs:2390:34
     |
     |
2390 |                     .map_or_else(String::new, |s| format!(" `{}`", s));
     |                                  ^^^^^^^^^^^ expected function that takes 1 argument
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0593`.
error: could not compile `rustc_typeck`
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_macros" "-p" "rustc_attr" "-p" "rustc_index" "-p" "rustc_parse_format" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_arena" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_ast_passes" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_passes" "-p" "rustc_symbol_mangling" "-p" "rustc_typeck" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_incremental" "-p" "rustc_traits" "-p" "rustc_ty_utils" "-p" "rustc_errors" "-p" "rustc_ast_pretty" "-p" "rustc_serialize" "-p" "rustc_target" "-p" "rustc_hir_pretty" "-p" "rustc_save_analysis" "-p" "rustc_error_codes" "-p" "rustc_feature" "-p" "rustc_session" "-p" "rustc_hir" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_metadata" "-p" "rustc_span" "-p" "rustc_plugin_impl" "-p" "rustc_data_structures" "-p" "rustc_ast" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:37
