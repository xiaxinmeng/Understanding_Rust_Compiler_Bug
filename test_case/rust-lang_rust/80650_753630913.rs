plain
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0308]: mismatched types
    --> compiler/rustc_ast/src/ast.rs:1099:37
     |
1095 |             match block.stmts.last().map(|last_stmt| &last_stmt.kind) {
     |                   --------------------------------------------------- this expression has type `Option<&ast::StmtKind>`
...
1099 |                 Some(StmtKind::Semi(Expr { kind: ExprKind::Ret(_), .. })) => true,
     |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `P`, found struct `ast::Expr`
     |
     = note: expected struct `P<ast::Expr>`
                found struct `ast::Expr`
    Checking tracing-serde v0.1.2
    Checking gsgdt v0.1.2
    Checking rls-span v0.5.2
    Checking rls-data v0.19.0
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_hir" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_macros" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_symbol_mangling" "-p" "rustc_index" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_apfloat" "-p" "rustc_data_structures" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_ty_utils" "-p" "rustc_expand" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_ast_passes" "-p" "rustc_ast_lowering" "-p" "rustc_mir_build" "-p" "rustc_builtin_macros" "-p" "rustc_typeck" "-p" "rustc_hir_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_metadata" "-p" "rustc_lint" "-p" "rustc_parse" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:57
