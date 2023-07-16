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
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0063]: missing field `erased` in initializer of `rustc_middle::mir::TerminatorKind<'_>`
  --> compiler/rustc_mir/src/dataflow/framework/tests.rs:37:9
37 |         mir::TerminatorKind::Call {
37 |         mir::TerminatorKind::Call {
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ missing `erased`

error[E0063]: missing field `erased` in initializer of `rustc_middle::mir::TerminatorKind<'_>`
  --> compiler/rustc_mir/src/dataflow/framework/tests.rs:50:9
50 |         mir::TerminatorKind::Call {
50 |         mir::TerminatorKind::Call {
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ missing `erased`

error[E0063]: missing field `erased` in initializer of `rustc_middle::mir::TerminatorKind<'_>`
    |
150 |             TerminatorKind::Call {
150 |             TerminatorKind::Call {
    |             ^^^^^^^^^^^^^^^^^^^^ missing `erased`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0063`.
error: could not compile `rustc_mir`
error: could not compile `rustc_mir`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_symbol_mangling" "-p" "rustc_apfloat" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_feature" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_hir" "-p" "rustc_incremental" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_fs_util" "-p" "rustc_span" "-p" "rustc_session" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_ast_passes" "-p" "rustc_query_impl" "-p" "rustc_expand" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_builtin_macros" "-p" "rustc_resolve" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_mir_build" "-p" "rustc_save_analysis" "-p" "rustc_typeck" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_hir_pretty" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:34
