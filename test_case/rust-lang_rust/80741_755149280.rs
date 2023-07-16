plain
    Checking rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
    Checking rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
    Checking rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0382]: borrow of moved value: `crate_dir`
    --> compiler/rustc_incremental/src/persist/fs.rs:213:17
     |
198  |     let crate_dir = crate_path(sess, crate_name, crate_disambiguator);
     |         --------- move occurs because `crate_dir` has type `PathBuf`, which does not implement the `Copy` trait
...
208  |     let crate_dir = match std_fs::canonicalize(crate_dir) {
     |                                                --------- value moved here
...
213  |                 crate_dir.display(),
     |                 ^^^^^^^^^^^^^^^^^^^ value borrowed here after move
     |
     = note: borrow occurs due to deref coercion to `std::path::Path`
note: deref defined here
    --> /checkout/library/std/src/path.rs:1504:5
1504 |     type Target = Path;
     |     ^^^^^^^^^^^^^^^^^^^


error[E0382]: borrow of moved value: `path`
    --> compiler/rustc_incremental/src/persist/load.rs:153:33
     |
145  |                     let path = in_incr_comp_dir_sess(sess, file_name);
     |                         ---- move occurs because `path` has type `PathBuf`, which does not implement the `Copy` trait
146  |                     if fs::metadata(path).is_err() {
     |                                     ---- value moved here
153  |                                 path.display()
153  |                                 path.display()
     |                                 ^^^^^^^^^^^^^^ value borrowed here after move
     |
     = note: borrow occurs due to deref coercion to `std::path::Path`
note: deref defined here
    --> /checkout/library/std/src/path.rs:1504:5
1504 |     type Target = Path;
     |     ^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rustc_incremental`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_serialize" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_ast_pretty" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_index" "-p" "rustc_symbol_mangling" "-p" "rustc_incremental" "-p" "rustc_apfloat" "-p" "rustc_fs_util" "-p" "rustc_ast" "-p" "rustc_span" "-p" "rustc_macros" "-p" "rustc_target" "-p" "rustc_errors" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "rustc_infer" "-p" "coverage_test_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_plugin_impl" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_mir_build" "-p" "rustc_ty_utils" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_typeck" "-p" "rustc_traits" "-p" "rustc_builtin_macros" "-p" "rustc_error_codes" "-p" "rustc_lint" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:12
