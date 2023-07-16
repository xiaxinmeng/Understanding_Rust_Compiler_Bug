plain
    Checking rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
    Checking rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
    Checking rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0277]: the trait bound `std::cell::Ref<'_, PathBuf>: AsRef<std::path::Path>` is not satisfied
    --> compiler/rustc_incremental/src/persist/fs.rs:390:46
     |
390  |     let sess_dir_iterator = std_fs::read_dir(sess.incr_comp_session_dir())?;
     |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `AsRef<std::path::Path>` is not implemented for `std::cell::Ref<'_, PathBuf>`
    ::: /checkout/library/std/src/fs.rs:2072:20
     |
     |
2072 | pub fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir> {
     |                    ----------- required by this bound in `read_dir`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_incremental`
error: could not compile `rustc_incremental`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_ast" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_typeck" "-p" "rustc_arena" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_incremental" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_privacy" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_serialize" "-p" "rustc_target" "-p" "rustc_lint" "-p" "rustc_errors" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_span" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_feature" "-p" "rustc_ast_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:13
