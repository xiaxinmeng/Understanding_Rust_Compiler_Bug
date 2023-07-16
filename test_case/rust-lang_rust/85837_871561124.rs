plain
    Checking chalk-engine v0.55.0
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0412]: cannot find type `StableCrateId` in this scope
   --> compiler/rustc_middle/src/middle/cstore.rs:210:50
    |
210 |     fn stable_crate_id(&self, cnum: CrateNum) -> StableCrateId;
    |
help: consider importing one of these items
    |
5   | use crate::ty::context::StableCrateId;
---
For more information about this error, try `rustc --explain E0412`.
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_index" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lexer" "-p" "rustc_lint_defs" "-p" "rustc_attr" "-p" "rustc_macros" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_parse_format" "-p" "rustc_parse" "-p" "rustc_data_structures" "-p" "rustc_hir" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_arena" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_apfloat" "-p" "rustc_query_system" "-p" "rustc_error_codes" "-p" "rustc_mir_build" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_hir_pretty" "-p" "rustc_ast" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_interface" "-p" "rustc_incremental" "-p" "rustc_query_impl" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_feature" "-p" "rustc_target" "-p" "rustc_ast_pretty" "-p" "rustc_span" "-p" "rustc_errors" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:46
