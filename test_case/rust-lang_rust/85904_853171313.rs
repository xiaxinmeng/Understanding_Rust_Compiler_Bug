plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 18 elements, found one with 17 elements
7   | | }
    | |_- in this expansion of `static_assert_size!`
    | 
    | 
   ::: compiler/rustc_middle/src/dep_graph/dep_node.rs:288:1
    |
288 |   static_assert_size!(DepNode, 18);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_mir" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_graphviz" "-p" "rustc_infer" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_apfloat" "-p" "rustc_macros" "-p" "coverage_test_macros" "-p" "rustc_error_codes" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_serialize" "-p" "rustc_typeck" "-p" "rustc_arena" "-p" "rustc_ty_utils" "-p" "rustc_target" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_incremental" "-p" "rustc_ast_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_span" "-p" "rustc_save_analysis" "-p" "rustc_mir_build" "-p" "rustc_ast" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_session" "-p" "rustc_data_structures" "-p" "rustc_hir_pretty" "-p" "rustc_lint" "-p" "rustc_feature" "-p" "rustc_hir" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:55
