plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error[E0532]: expected tuple struct or tuple variant, found struct variant `ExpnKind::Macro`
   --> compiler/rustc_mir/src/transform/coverage/mod.rs:590:17
    |
590 |                 ExpnKind::Macro(..) => {}
    | 
   ::: /checkout/compiler/rustc_span/src/hygiene.rs:845:5
    |
845 |     Macro {
845 |     Macro {
    |     ----- `ExpnKind::Macro` defined here
help: use struct pattern syntax instead
    |
    |
590 |                 ExpnKind::Macro { /* fields */ } => {}
help: consider importing one of these items instead
    |
    |
11  | use crate::transform::const_prop::DefKind::Macro;
11  | use rustc_hir::def::DefKind::Macro;
    |

    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_mir_build" "-p" "rustc_arena" "-p" "rustc_trait_selection" "-p" "rustc_macros" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_apfloat" "-p" "rustc_serialize" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_save_analysis" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_span" "-p" "rustc_lint" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_parse" "-p" "rustc_hir" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_ast" "-p" "rustc_ast_pretty" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_ast_lowering" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_incremental" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_query_impl" "-p" "rustc_error_codes" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_data_structures" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:08
