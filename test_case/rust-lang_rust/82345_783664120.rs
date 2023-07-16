plain
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
error: unused import: `AtomicU64`
 --> compiler/rustc_query_system/src/dep_graph/graph.rs:5:46
  |
5 | use rustc_data_structures::sync::{AtomicU32, AtomicU64, Lock, Lrc, RwLock};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `std::env`
error: unused import: `std::env`
  --> compiler/rustc_query_system/src/dep_graph/graph.rs:12:5
12 | use std::env;
   |     ^^^^^^^^


error: unused import: `super::debug::EdgeFilter`
  --> compiler/rustc_query_system/src/dep_graph/graph.rs:17:5
17 | use super::debug::EdgeFilter;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 3 previous errors
error: aborting due to 3 previous errors

error: could not compile `rustc_query_system`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_target" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_data_structures" "-p" "rustc_fs_util" "-p" "rustc_macros" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_hir" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_symbol_mangling" "-p" "rustc_index" "-p" "rustc_errors" "-p" "rustc_driver" "-p" "rustc_hir_pretty" "-p" "rustc_mir" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "coverage_test_macros" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_mir_build" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_passes" "-p" "rustc_typeck" "-p" "rustc_query_impl" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_plugin_impl" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:58
