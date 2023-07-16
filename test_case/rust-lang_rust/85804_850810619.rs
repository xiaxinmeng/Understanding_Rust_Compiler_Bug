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
  |
2 | use rustc_data_structures::fingerprint::Fingerprint;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error[E0308]: mismatched types
  --> compiler/rustc_hir/src/tests.rs:15:53
   |
   |
15 |     let id0 = StableCrateId::new("foo", false, vec!["1"]);
   |                                                     |
   |                                                     expected struct `std::string::String`, found `&str`
   |                                                     expected struct `std::string::String`, found `&str`
   |                                                     help: try using a conversion method: `"1".to_string()`
error[E0308]: mismatched types
  --> compiler/rustc_hir/src/tests.rs:16:53
   |
   |
16 |     let id1 = StableCrateId::new("foo", false, vec!["2"]);
   |                                                     |
   |                                                     expected struct `std::string::String`, found `&str`
   |                                                     expected struct `std::string::String`, found `&str`
   |                                                     help: try using a conversion method: `"2".to_string()`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_hir`
error: could not compile `rustc_hir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_error_codes" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_macros" "-p" "rustc_fs_util" "-p" "rustc_resolve" "-p" "rustc_arena" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_ast_passes" "-p" "rustc_index" "-p" "rustc_builtin_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_attr" "-p" "rustc_plugin_impl" "-p" "rustc_typeck" "-p" "rustc_target" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_lint" "-p" "rustc_ast_pretty" "-p" "rustc_ast" "-p" "rustc_serialize" "-p" "rustc_hir_pretty" "-p" "rustc_parse" "-p" "rustc_span" "-p" "rustc_hir" "-p" "rustc_metadata" "-p" "rustc_data_structures" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_save_analysis" "-p" "rustc_errors" "-p" "rustc_session" "-p" "rustc_mir_build" "-p" "rustc_feature" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:56
