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
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0283]: type annotations needed
 --> compiler/rustc_data_structures/src/thin_vec/tests.rs:5:64
  |
5 |     assert_eq!(std::iter::empty().collect::<ThinVec<String>>().into(), Vec::<String>::new());
  |                |                                               |
  |                |                                               |
  |                |                                               cannot infer type for type parameter `T` declared on the trait `Into`
  |                this method call resolves to `T`
  |
  = note: cannot satisfy `thin_vec::ThinVec<std::string::String>: Into<_>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
error: could not compile `rustc_data_structures`
error: could not compile `rustc_data_structures`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_fs_util" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_symbol_mangling" "-p" "rustc_macros" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_feature" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_incremental" "-p" "rustc_session" "-p" "rustc_serialize" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_hir" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_typeck" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_builtin_macros" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_ast_lowering" "-p" "rustc_query_impl" "-p" "rustc_resolve" "-p" "rustc_mir_build" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_hir_pretty" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:00
