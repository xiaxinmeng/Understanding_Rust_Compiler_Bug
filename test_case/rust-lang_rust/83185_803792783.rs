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
    Checking chalk-ir v0.55.0
    Checking tracing v0.1.25
    Checking rustc_index v0.0.0 (/checkout/compiler/rustc_index)
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0592]: duplicate definitions with name `len`
  --> compiler/rustc_data_structures/src/tiny_list/tests.rs:7:5
   |
7  |     fn len(&self) -> usize {
   |     ^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `len`
  ::: compiler/rustc_data_structures/src/tiny_list.rs:61:5
   |
   |
61 |     fn len(&self) -> usize {
   |     ---------------------- other definition for `len`

error[E0592]: duplicate definitions with name `postdom_parent`
    |
    |
6   |     fn postdom_parent(&self, a: &T) -> Option<&T> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `postdom_parent`
   ::: compiler/rustc_data_structures/src/transitive_relation.rs:328:5
    |
    |
328 |     fn postdom_parent(&self, a: &T) -> Option<&T> {
    |     --------------------------------------------- other definition for `postdom_parent`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0592`.
error: could not compile `rustc_data_structures`
error: could not compile `rustc_data_structures`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_symbol_mangling" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_feature" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_serialize" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_target" "-p" "rustc_fs_util" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_data_structures" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_session" "-p" "rustc_hir" "-p" "rustc_driver" "-p" "rustc_parse" "-p" "rustc_mir_build" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lint" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_error_codes" "-p" "rustc_typeck" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_hir_pretty" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_query_impl" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_passes" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:51
