plain
    Checking rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
    Checking rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
    Checking rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0425]: cannot find value `e` in this scope
  --> compiler/rustc_incremental/src/persist/file_format.rs:58:32
   |
58 |         Err(err) => return Err(e),
   |                                ^ not found in this scope

error[E0277]: the trait bound `std::cell::Ref<'_, PathBuf>: AsRef<std::path::Path>` is not satisfied
    --> compiler/rustc_incremental/src/persist/fs.rs:390:46
     |
390  |     let sess_dir_iterator = std_fs::read_dir(&sess.incr_comp_session_dir())?;
     |                                              |
     |                                              |
     |                                              the trait `AsRef<std::path::Path>` is not implemented for `std::cell::Ref<'_, PathBuf>`
     |                                              help: consider adding dereference here: `&*sess.incr_comp_session_dir()`
    ::: /checkout/library/std/src/fs.rs:2072:20
     |
     |
2072 | pub fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir> {
     |                    ----------- required by this bound in `read_dir`
     |
     = note: required because of the requirements on the impl of `AsRef<std::path::Path>` for `&std::cell::Ref<'_, PathBuf>`

error[E0615]: attempted to take value of method `kind` on type `std::io::Error`
   --> compiler/rustc_incremental/src/persist/save.rs:106:25
    |
106 |         Err(err) if err.kind == io::ErrorKind::NotFound => (),
    |                         ^^^^ method, not a field
help: use parentheses to call the method
    |
    |
106 |         Err(err) if err.kind() == io::ErrorKind::NotFound => (),

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0425, E0615.
Some errors have detailed explanations: E0277, E0425, E0615.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_incremental`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_data_structures" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_graphviz" "-p" "rustc_hir" "-p" "rustc_hir_pretty" "-p" "rustc_ast_pretty" "-p" "rustc_middle" "-p" "rustc_attr" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_parse" "-p" "rustc_ast" "-p" "rustc_error_codes" "-p" "rustc_feature" "-p" "rustc_plugin_impl" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_fs_util" "-p" "rustc_symbol_mangling" "-p" "rustc_expand" "-p" "rustc_traits" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_incremental" "-p" "rustc_mir_build" "-p" "rustc_ast_passes" "-p" "rustc_ast_lowering" "-p" "rustc_lint" "-p" "rustc_session" "-p" "rustc_metadata" "-p" "rustc_serialize" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:15
