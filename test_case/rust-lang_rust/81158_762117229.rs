plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0277]: `rustc_middle::hir::place::Place<'_>` doesn't implement `std::fmt::Display`
   --> compiler/rustc_mir/src/borrow_check/diagnostics/mutability_errors.rs:254:29
251 | /                         format!(
251 | /                         format!(
252 | |                             "calling `{}` requires mutable borrow due to mutation of `{}`",
253 | |                             self.local_names[local].unwrap(),
254 | |                             symbol
    | |                             ^^^^^^ `rustc_middle::hir::place::Place<'_>` cannot be formatted with the default formatter
    | |_________________________- in this macro invocation (#1)
    | 
   ::: /checkout/library/alloc/src/macros.rs:108:1
    |
    |
108 | / macro_rules! format {
109 | |     ($($arg:tt)*) => {{
110 | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
    | |                                       ---------------------------------------- in this macro invocation (#2)
112 | |     }}
113 | | }
113 | | }
    | |_- in this expansion of `format!` (#1)
   ::: /checkout/library/core/src/macros/mod.rs:749:5
    |
749 | /     macro_rules! format_args {
749 | /     macro_rules! format_args {
750 | |         ($fmt:expr) => {{ /* compiler built-in */ }};
751 | |         ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
752 | |     }
    | |_____- in this expansion of `$crate::__export::format_args!` (#2)
    |
    = help: the trait `std::fmt::Display` is not implemented for `rustc_middle::hir::place::Place<'_>`
    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    = note: required by `std::fmt::Display::fmt`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_mir`
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_target" "-p" "rustc_symbol_mangling" "-p" "rustc_serialize" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_feature" "-p" "rustc_ast_pretty" "-p" "rustc_apfloat" "-p" "rustc_ast" "-p" "rustc_macros" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_hir_pretty" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_ty_utils" "-p" "rustc_ast_passes" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_typeck" "-p" "rustc_expand" "-p" "rustc_mir_build" "-p" "rustc_save_analysis" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_metadata" "-p" "rustc_plugin_impl" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:00
