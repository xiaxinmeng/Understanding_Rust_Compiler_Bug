plain
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking chalk-engine v0.55.0
error: associated function is never used: `register_dep_node_debug_str`
   --> compiler/rustc_query_system/src/dep_graph/graph.rs:395:19
    |
395 |     pub(crate) fn register_dep_node_debug_str<F>(&self, dep_node: DepNode<K>, debug_str_gen: F)
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_query_system`


To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_macros" "-p" "rustc_ast" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_span" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_hir" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_resolve" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_mir_build" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_query_impl" "-p" "rustc_ast_lowering" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_serialize" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_parse" "-p" "rustc_save_analysis" "-p" "rustc_errors" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_ast_pretty" "-p" "rustc_hir_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:49
