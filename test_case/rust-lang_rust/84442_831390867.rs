plain
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error: unexpected closing delimiter: `}`
   --> compiler/rustc_expand/src/config.rs:474:5
    |
455 |     fn in_cfg(&self, attrs: &[Attribute]) -> bool {
    |                                                   - this opening brace...
473 |         })
473 |         })
    |         - ...matches this closing brace
    |     ^ unexpected closing delimiter

error: mismatched closing delimiter: `}`
   --> compiler/rustc_expand/src/config.rs:473:9
   --> compiler/rustc_expand/src/config.rs:473:9
    |
456 |         attrs.iter().all(|attr| {
...
473 |         })
    |         ^ mismatched closing delimiter


error: mismatched closing delimiter: `)`
   --> compiler/rustc_expand/src/config.rs:473:10
    |
245 | impl<'a> StripUnconfigured<'a> {
...
473 |         })
    |          ^ mismatched closing delimiter


error: aborting due to 3 previous errors

error: could not compile `rustc_expand`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_middle" "-p" "rustc_macros" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_parse" "-p" "rustc_mir_build" "-p" "rustc_hir_pretty" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_lint" "-p" "rustc_ast_pretty" "-p" "rustc_feature" "-p" "rustc_serialize" "-p" "rustc_target" "-p" "rustc_error_codes" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint_defs" "-p" "rustc_hir" "-p" "rustc_errors" "-p" "rustc_span" "-p" "rustc_interface" "-p" "rustc_ast_passes" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_lowering" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_expand" "-p" "rustc_symbol_mangling" "-p" "rustc_incremental" "-p" "rustc_builtin_macros" "-p" "rustc_query_impl" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_metadata" "-p" "rustc_ast" "-p" "rustc_data_structures" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:36
