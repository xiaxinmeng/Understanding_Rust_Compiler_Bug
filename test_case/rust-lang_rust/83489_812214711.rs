plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
error[E0425]: cannot find value `hir_id` in this scope
   --> compiler/rustc_typeck/src/check/demand.rs:383:60
    |
383 |             let parent_id = self.tcx.hir().get_parent_node(hir_id);


error[E0425]: cannot find value `hir_id` in this scope
   --> compiler/rustc_typeck/src/check/demand.rs:389:44
    |
389 |                 return else_expr.hir_id == hir_id;

    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0308]: mismatched types
   --> compiler/rustc_typeck/src/check/demand.rs:681:65
   --> compiler/rustc_typeck/src/check/demand.rs:681:65
    |
681 | ...                   } else if self.is_else_if_block(expr.hir_id) {
    |                                                       ^^^^^^^^^^^ expected `&rustc_hir::Expr<'_>`, found struct `HirId`

error[E0308]: `if` and `else` have incompatible types
    |
111 |   / macro_rules! format {
112 |   |     ($($arg:tt)*) => {{
    |  _|_______________________-
    |  _|_______________________-
113 | | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
114 | | |         res
115 | | |     }}
    | | |     ^
    | | |     |
    | |_|_____expected `()`, found struct `std::string::String`
    |   |     expected because of this
116 |   | }
    |   |_- in this expansion of `format!`
   ::: compiler/rustc_typeck/src/check/demand.rs:684:40
    |
    |
684 |     ...                   } else if let Some(expr) = self.maybe_get_block_expr(expr) {
    |  ________________________________-
685 | |   ...                       format!("*{}", sm.span_to_snippet(expr.span).unwrap_or(code));
    | |                                                                                          - help: consider removing this semicolon
686 | |   ...                   } else {
687 | |   ...                       format!("*{}", code)
688 | |   ...                   };
688 | |   ...                   };
    | |_________________________- `if` and `else` have incompatible types
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_target" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_feature" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_session" "-p" "rustc_apfloat" "-p" "rustc_span" "-p" "rustc_macros" "-p" "rustc_hir" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_ast_pretty" "-p" "rustc_data_structures" "-p" "rustc_index" "-p" "rustc_fs_util" "-p" "rustc_ast" "-p" "rustc_symbol_mangling" "-p" "rustc_serialize" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_query_impl" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_expand" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "rustc_infer" "-p" "rustc_ty_utils" "-p" "rustc_privacy" "-p" "rustc_ast_passes" "-p" "rustc_trait_selection" "-p" "rustc_ast_lowering" "-p" "rustc_typeck" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_hir_pretty" "-p" "rustc_mir_build" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:05
