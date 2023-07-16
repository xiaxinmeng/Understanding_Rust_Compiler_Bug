plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0425]: cannot find value `to_numeric` in this scope
     |
     |
1038 |             if to_numeric && !matches!(t_cast.kind(), ty::Uint(ty::UintTy::Usize)) {

    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0061]: this function takes 1 argument but 2 arguments were supplied
    --> compiler/rustc_typeck/src/check/expr.rs:1046:30
    --> compiler/rustc_typeck/src/check/expr.rs:1046:30
     |
1046 | ...                   .help(
     |                        ^^^^ expected 1 argument
1047 | ...                       expr.span,
     |                           ---------
1048 | ...                       "pointers should only be cast to `usize`",
     |
note: associated function defined here
    --> /checkout/compiler/rustc_errors/src/diagnostic_builder.rs:261:21
     |
     |
261  |     forward!(pub fn help(&mut self, msg: &str) -> &mut Self);


error[E0277]: `rustc_span::Span` doesn't implement `std::fmt::Display`
     |
     |
1053 |   ...                   &format!("{} as usize as {}", e.span, t_cast),
     |                          |                            |
     |                          |                            |
     |                          |                            `rustc_span::Span` cannot be formatted with the default formatter
     |                          in this macro invocation (#1)
    ::: /checkout/library/alloc/src/macros.rs:111:1
     |
111  | / macro_rules! format {
112  | |     ($($arg:tt)*) => {{
---
     | |_- in this expansion of `format!` (#1)
     | 
    ::: /checkout/library/core/src/macros/mod.rs:739:5
     |
739  | /     macro_rules! format_args {
740  | |         ($fmt:expr) => {{ /* compiler built-in */ }};
741  | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
742  | |     }
     | |_____- in this expansion of `$crate::__export::format_args!` (#2)
     = help: the trait `std::fmt::Display` is not implemented for `rustc_span::Span`
     = help: the trait `std::fmt::Display` is not implemented for `rustc_span::Span`
     = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
     = note: required by `std::fmt::Display::fmt`
error[E0308]: mismatched types
    --> compiler/rustc_typeck/src/check/expr.rs:1053:33
     |
     |
1053 | ...                   &format!("{} as usize as {}", e.span, t_cast),
     |                       |
     |                       |
     |                       expected struct `std::string::String`, found `&std::string::String`
     |                       help: consider removing the borrow: `format!("{} as usize as {}", e.span, t_cast)`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0061, E0277, E0308, E0425.
For more information about an error, try `rustc --explain E0061`.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_middle" "-p" "rustc_macros" "-p" "rustc_arena" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_index" "-p" "rustc_mir" "-p" "rustc_graphviz" "-p" "coverage_test_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_span" "-p" "rustc_hir" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_ast_pretty" "-p" "rustc_interface" "-p" "rustc_ast_passes" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_traits" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_typeck" "-p" "rustc_resolve" "-p" "rustc_query_impl" "-p" "rustc_ast_lowering" "-p" "rustc_mir_build" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_passes" "-p" "rustc_ty_utils" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_serialize" "-p" "rustc_lint" "-p" "rustc_feature" "-p" "rustc_save_analysis" "-p" "rustc_hir_pretty" "-p" "rustc_data_structures" "-p" "rustc_ast" "-p" "rustc_errors" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_metadata" "-p" "rustc_plugin_impl" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:56
