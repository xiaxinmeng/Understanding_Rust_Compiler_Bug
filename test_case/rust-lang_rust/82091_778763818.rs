plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
error: missing condition for `if` expression
    --> compiler/rustc_mir/src/transform/check_consts/validation.rs:1055:19
     |
1055 |                 if matches!(inner_ty.kind(), ty::Ref(..)) =  {
     |                   ^ expected if condition here
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0308]: mismatched types
    --> compiler/rustc_mir/src/transform/check_consts/validation.rs:1055:20
     |
     |
1055 |                   if matches!(inner_ty.kind(), ty::Ref(..)) =  {
1056 | |                     return Some(place_base);
1057 | |                 } else {
1057 | |                 } else {
     | |_________________^ expected `bool`, found `()`
     |
help: you might have meant to use pattern matching
     |
1055 |                 if let matches!(inner_ty.kind(), ty::Ref(..)) =  {
     |                    ^^^
help: you might have meant to compare for equality
     |
1055 |                 if matches!(inner_ty.kind(), ty::Ref(..)) ==  {

error: unreachable expression
    --> compiler/rustc_mir/src/transform/check_consts/validation.rs:1055:20
     |
     |
1055 |                   if matches!(inner_ty.kind(), ty::Ref(..)) =  {
1056 | |                     return Some(place_base);
     | |                     ----------------------- any code following this expression is unreachable
1057 | |                 } else {
     | |_________________^ unreachable expression
     | |_________________^ unreachable expression
     |
     = note: `-D unreachable-code` implied by `-D warnings`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_mir`
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_arena" "-p" "rustc_privacy" "-p" "rustc_ast_passes" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_typeck" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_symbol_mangling" "-p" "rustc_trait_selection" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_session" "-p" "rustc_span" "-p" "rustc_plugin_impl" "-p" "rustc_ast" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_lint" "-p" "rustc_feature" "-p" "rustc_hir_pretty" "-p" "rustc_errors" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_serialize" "-p" "rustc_metadata" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_target" "-p" "rustc_save_analysis" "-p" "rustc_ast_pretty" "-p" "rustc_data_structures" "-p" "rustc_hir" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:16
