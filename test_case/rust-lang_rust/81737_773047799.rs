plain
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
error: named argument never used
    --> compiler/rustc_typeck/src/check/expr.rs:1474:41
     |
1471 | ...                   "`{adt}::{variant}(/* fields */)`",
     |                       ---------------------------------- formatting specifier missing
...
1474 | ...                   kind_name = kind_name
     |                                   ^^^^^^^^^ named argument never used
error: named argument never used
    --> compiler/rustc_typeck/src/check/expr.rs:1489:80
     |
     |
1489 |                         format!("`{adt}(/* fields */)`", adt = ty, kind_name = kind_name),
     |                                 -----------------------                        ^^^^^^^^^ named argument never used
     |                                 formatting specifier missing

    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0308]: mismatched types
error[E0308]: mismatched types
    --> /checkout/library/alloc/src/macros.rs:113:9
     |
110  | / macro_rules! format {
111  | |     ($($arg:tt)*) => {{
112  | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
113  | |         res
     | |         ^^^ expected `&str`, found struct `std::string::String`
115  | | }
     | |_- in this expansion of `format!`
     | 
    ::: compiler/rustc_typeck/src/check/expr.rs:1465:25
    ::: compiler/rustc_typeck/src/check/expr.rs:1465:25
     |
1465 | /                         format!(
1466 | |                             "`{adt}::{variant}` is a tuple {kind_name}, use the appropriate syntax",
1467 | |                             adt = ty,
1468 | |                             variant = variant.ident
     | |_________________________- in this macro invocation

error[E0308]: mismatched types
    --> /checkout/library/alloc/src/macros.rs:113:9
    --> /checkout/library/alloc/src/macros.rs:113:9
     |
110  | / macro_rules! format {
111  | |     ($($arg:tt)*) => {{
112  | |         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
113  | |         res
     | |         ^^^ expected `&str`, found struct `std::string::String`
115  | | }
     | |_- in this expansion of `format!`
     | 
    ::: compiler/rustc_typeck/src/check/expr.rs:1484:25
    ::: compiler/rustc_typeck/src/check/expr.rs:1484:25
     |
1484 | /                         format!(
1485 | |                             "`{adt}` is a tuple {kind_name}, use the appropriate syntax",
1486 | |                             adt = ty,
1487 | |                             kind_name = kind_name
     | |_________________________- in this macro invocation

error: aborting due to 4 previous errors


For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_mir" "-p" "rustc_macros" "-p" "rustc_infer" "-p" "rustc_attr" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_graphviz" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_ast_pretty" "-p" "rustc_error_codes" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_lowering" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_ty_utils" "-p" "rustc_mir_build" "-p" "rustc_incremental" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_typeck" "-p" "rustc_symbol_mangling" "-p" "rustc_data_structures" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_errors" "-p" "rustc_serialize" "-p" "rustc_hir" "-p" "rustc_save_analysis" "-p" "rustc_metadata" "-p" "rustc_ast" "-p" "rustc_feature" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_lint" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:24
