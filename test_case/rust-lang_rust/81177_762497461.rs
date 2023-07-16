plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0061]: this function takes 1 argument but 0 arguments were supplied
  --> compiler/rustc_expand/src/parse/tests.rs:32:52
   |
32 |     new_parser_from_source_str(sess, name, source).parse_item()
   |                                                    |
   |                                                    expected 1 argument

error[E0061]: this function takes 1 argument but 0 arguments were supplied
error[E0061]: this function takes 1 argument but 0 arguments were supplied
  --> compiler/rustc_expand/src/parse/tests.rs:47:58
   |
47 |     with_error_checking_parse(source_str, &sess(), |p| p.parse_item())
   |                                                          |
   |                                                          expected 1 argument

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustc_expand`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_index" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_symbol_mangling" "-p" "rustc_fs_util" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_apfloat" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_feature" "-p" "rustc_query_system" "-p" "rustc_hir" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_target" "-p" "rustc_data_structures" "-p" "rustc_session" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_mir_build" "-p" "rustc_typeck" "-p" "rustc_passes" "-p" "rustc_lint" "-p" "rustc_hir_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_mir" "-p" "coverage_test_macros" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:25
