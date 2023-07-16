plain
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0433]: failed to resolve: use of undeclared type `FileName`
   --> compiler/rustc_interface/src/passes.rs:634:25
    |
634 |             .map(|path| FileName::Real(RealFileName::Named(path)))
    |
help: consider importing one of these items
    |
1   | use crate::interface::FileName;
1   | use crate::interface::FileName;
    |
1   | use rustc_span::FileName;
    |

error[E0433]: failed to resolve: use of undeclared type `RealFileName`
   --> compiler/rustc_interface/src/passes.rs:634:40
    |
634 |             .map(|path| FileName::Real(RealFileName::Named(path)))
    |
help: consider importing this enum
    |
1   | use rustc_span::RealFileName;
---
For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_interface`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_feature" "-p" "rustc_arena" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_symbol_mangling" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_serialize" "-p" "rustc_fs_util" "-p" "rustc_macros" "-p" "rustc_ast" "-p" "rustc_data_structures" "-p" "rustc_span" "-p" "rustc_hir" "-p" "rustc_session" "-p" "rustc_driver" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_query_impl" "-p" "rustc_builtin_macros" "-p" "rustc_privacy" "-p" "rustc_expand" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_passes" "-p" "rustc_typeck" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_save_analysis" "-p" "rustc_mir_build" "-p" "rustc_parse" "-p" "rustc_metadata" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:27
