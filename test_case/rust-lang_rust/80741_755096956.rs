plain
    Checking petgraph v0.5.1
error[E0433]: failed to resolve: use of undeclared crate or module `fs`
   --> compiler/rustc_llvm/build.rs:208:19
    |
208 |         } else if fs::metadata(Path::new(lib)).is_ok() {
    |                   ^^ use of undeclared crate or module `fs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_llvm`
error: could not compile `rustc_llvm`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_plugin_impl" "-p" "rustc_mir" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_graphviz" "-p" "rustc_apfloat" "-p" "coverage_test_macros" "-p" "rustc_infer" "-p" "rustc_attr" "-p" "rustc_index" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_resolve" "-p" "rustc_arena" "-p" "rustc_builtin_macros" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_traits" "-p" "rustc_typeck" "-p" "rustc_mir_build" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_lowering" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_error_codes" "-p" "rustc_data_structures" "-p" "rustc_parse" "-p" "rustc_feature" "-p" "rustc_lint" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_span" "-p" "rustc_hir_pretty" "-p" "rustc_ast" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:50
