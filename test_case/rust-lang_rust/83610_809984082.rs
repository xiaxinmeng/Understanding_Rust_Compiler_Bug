plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unused import: `rustc_data_structures::fx::FxHashMap`
  |
  |
7 | use rustc_data_structures::fx::FxHashMap;
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused import: `rustc_middle::ty::query::Providers`
   |
12 | use rustc_middle::ty::query::Providers;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error[E0308]: mismatched types
   --> compiler/rustc_codegen_llvm/src/lib.rs:268:13
    |
268 |             crate::llvm_util::target_cpu(tcx.sess),
    |             |
    |             |
    |             expected struct `std::string::String`, found `&str`
    |             help: try using a conversion method: `crate::llvm_util::target_cpu(tcx.sess).to_string()`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_codegen_llvm`
error: could not compile `rustc_codegen_llvm`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_symbol_mangling" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_arena" "-p" "rustc_ast_lowering" "-p" "rustc_attr" "-p" "rustc_macros" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_ast_passes" "-p" "rustc_traits" "-p" "rustc_ty_utils" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_fs_util" "-p" "rustc_index" "-p" "rustc_incremental" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_apfloat" "-p" "rustc_session" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_ast_pretty" "-p" "rustc_span" "-p" "rustc_metadata" "-p" "rustc_lint" "-p" "rustc_typeck" "-p" "rustc_save_analysis" "-p" "rustc_ast" "-p" "rustc_target" "-p" "rustc_mir_build" "-p" "rustc_errors" "-p" "rustc_serialize" "-p" "rustc_data_structures" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_feature" "-p" "rustc_hir" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:27
