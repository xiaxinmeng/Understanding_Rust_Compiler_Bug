plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: the feature `cmp_min_max_by` has been stable since 1.50.0 and no longer requires an attribute to enable
   |
   |
32 | #![feature(cmp_min_max_by)]
   |
   |
   = note: `-D stable-features` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_middle`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_symbol_mangling" "-p" "rustc_index" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_hir" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_data_structures" "-p" "rustc_errors" "-p" "rustc_target" "-p" "rustc_serialize" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_macros" "-p" "rustc_fs_util" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_span" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_typeck" "-p" "rustc_privacy" "-p" "rustc_mir_build" "-p" "rustc_resolve" "-p" "rustc_ast_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_ty_utils" "-p" "rustc_expand" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "rustc_metadata" "-p" "rustc_lint" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:33
