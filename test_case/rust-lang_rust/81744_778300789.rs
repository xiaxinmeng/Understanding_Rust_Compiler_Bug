plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unused variable: `impl1_self`
   |
   |
73 |     let impl1_self = tcx.type_of(impl1_def_id);
   |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_impl1_self`
   |
   = note: `-D unused-variables` implied by `-D warnings`

error: unused variable: `impl2_self`
   |
   |
74 |     let impl2_self = tcx.type_of(impl2_def_id);
   |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_impl2_self`
error: aborting due to 2 previous errors

error: could not compile `rustc_trait_selection`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_macros" "-p" "rustc_fs_util" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_feature" "-p" "rustc_apfloat" "-p" "rustc_hir" "-p" "rustc_serialize" "-p" "rustc_index" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_errors" "-p" "rustc_target" "-p" "rustc_symbol_mangling" "-p" "rustc_incremental" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "rustc_hir_pretty" "-p" "rustc_interface" "-p" "rustc_typeck" "-p" "rustc_infer" "-p" "rustc_mir_build" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_expand" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_ast_lowering" "-p" "rustc_ast_passes" "-p" "rustc_ty_utils" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_metadata" "-p" "rustc_lint" "-p" "rustc_error_codes" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:10
