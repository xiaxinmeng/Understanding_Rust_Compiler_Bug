plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0382]: borrow of moved value: `path`
   --> compiler/rustc_codegen_llvm/src/back/lto.rs:500:48
    |
495 |             let path = incr_comp_session_dir.join(THIN_LTO_KEYS_INCR_COMP_FILE_NAME);
    |                 ---- move occurs because `path` has type `PathBuf`, which does not implement the `Copy` trait
...
499 |             let prev = if fs::metadata(path).is_ok() {
    |                                        ---- value moved here
500 |                 ThinLTOKeysMap::load_from_file(&path).ok()
    |                                                ^^^^^ value borrowed here after move
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustc_codegen_llvm`
error: could not compile `rustc_codegen_llvm`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_index" "-p" "rustc_target" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_apfloat" "-p" "rustc_data_structures" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_serialize" "-p" "rustc_symbol_mangling" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_fs_util" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_span" "-p" "rustc_session" "-p" "rustc_macros" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_parse" "-p" "rustc_error_codes" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_save_analysis" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_builtin_macros" "-p" "rustc_passes" "-p" "rustc_ty_utils" "-p" "rustc_resolve" "-p" "rustc_mir_build" "-p" "rustc_privacy" "-p" "rustc_typeck" "-p" "rustc_ast_lowering" "-p" "rustc_hir_pretty" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:36
