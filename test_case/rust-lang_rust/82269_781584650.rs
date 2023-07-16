plain
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error: unreachable pattern
  --> compiler/rustc_driver/src/pretty.rs:58:9
   |
58 |         _ => panic!("Should use call_with_pp_support_hir"),
   |
   |
   = note: `-D unreachable-patterns` implied by `-D warnings`
error: unreachable pattern
  --> compiler/rustc_driver/src/pretty.rs:81:9
   |
   |
81 |         _ => panic!("Should use call_with_pp_support"),

error: aborting due to 2 previous errors

error: could not compile `rustc_driver`
error: could not compile `rustc_driver`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_target" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_symbol_mangling" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_span" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_ast_pretty" "-p" "rustc_apfloat" "-p" "rustc_hir" "-p" "rustc_macros" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_ast" "-p" "rustc_index" "-p" "rustc_incremental" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_privacy" "-p" "rustc_mir_build" "-p" "rustc_expand" "-p" "rustc_ast_lowering" "-p" "rustc_ast_passes" "-p" "rustc_typeck" "-p" "rustc_traits" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_resolve" "-p" "rustc_hir_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_lint" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_save_analysis" "-p" "rustc_error_codes" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:00
