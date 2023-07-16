plain
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0433]: failed to resolve: use of undeclared crate or module `fs`
   --> compiler/rustc_interface/src/util.rs:272:9
    |
272 |         fs::metadata(candidate).is_ok().then_some(candidate)
    |         ^^ use of undeclared crate or module `fs`
error[E0433]: failed to resolve: use of undeclared crate or module `fs`
   --> compiler/rustc_interface/src/util.rs:279:48
    |
    |
279 |     let path = current_dll_path().and_then(|s| fs::canonicalize(s).ok());
    |                                                ^^ use of undeclared crate or module `fs`
error[E0433]: failed to resolve: use of undeclared crate or module `fs`
   --> compiler/rustc_interface/src/util.rs:400:13
    |
    |
400 |             fs::metadata(f).is_ok()
    |             ^^ use of undeclared crate or module `fs`
error[E0433]: failed to resolve: use of undeclared crate or module `fs`
   --> compiler/rustc_interface/src/util.rs:417:13
    |
    |
417 |     let d = fs::read_dir(sysroot).unwrap_or_else(|e| {
    |             ^^ use of undeclared crate or module `fs`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0433`.
error: could not compile `rustc_interface`
error: could not compile `rustc_interface`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_hir" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_ast_pretty" "-p" "rustc_serialize" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_lint" "-p" "rustc_attr" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_arena" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_expand" "-p" "rustc_resolve" "-p" "rustc_typeck" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_incremental" "-p" "rustc_ast_passes" "-p" "rustc_error_codes" "-p" "rustc_feature" "-p" "rustc_ast" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_span" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_data_structures" "-p" "rustc_metadata" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:46
