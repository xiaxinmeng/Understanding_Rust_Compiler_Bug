plain
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error: unused import: `DefPathData`
 --> compiler/rustc_mir/src/interpret/intrinsics/type_name.rs:2:30
  |
2 | use rustc_hir::definitions::{DefPathData, DisambiguatedDefPathData};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: aborting due to previous error

error: could not compile `rustc_mir`
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_symbol_mangling" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_target" "-p" "rustc_ast" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_serialize" "-p" "rustc_fs_util" "-p" "rustc_hir" "-p" "rustc_span" "-p" "rustc_macros" "-p" "rustc_data_structures" "-p" "rustc_driver" "-p" "rustc_mir_build" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_plugin_impl" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_expand" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_query_impl" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_ast_passes" "-p" "rustc_resolve" "-p" "rustc_metadata" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_lint" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:28
