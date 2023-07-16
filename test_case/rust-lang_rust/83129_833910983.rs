plain
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
error[E0026]: variant `thir::InlineAsmOperand::Const` does not have a field named `expr`
   --> compiler/rustc_mir_build/src/thir/visit.rs:116:31
116 |                     | Const { expr }
116 |                     | Const { expr }
    |                               ^^^^ variant `thir::InlineAsmOperand::Const` does not have this field

error[E0027]: pattern does not mention fields `value`, `span`
   --> compiler/rustc_mir_build/src/thir/visit.rs:116:23
116 |                     | Const { expr }
116 |                     | Const { expr }
    |                       ^^^^^^^^^^^^^^ missing fields `value`, `span`
help: include the missing fields in the pattern
    |
    |
116 |                     | Const { expr, value, span }
    |                                   ^^^^^^^^^^^^^^^
help: if you don't care about these missing fields, you can explicitly ignore them
116 |                     | Const { expr, .. }
    |                                   ^^^^^^

error: aborting due to 2 previous errors
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_macros" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_apfloat" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_errors" "-p" "rustc_index" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_data_structures" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_symbol_mangling" "-p" "rustc_driver" "-p" "rustc_mir_build" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_hir_pretty" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_query_impl" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_save_analysis" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:30
