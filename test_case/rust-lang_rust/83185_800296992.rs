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
    Checking rustc_lexer v0.1.0 (/checkout/compiler/rustc_lexer)
   Compiling indexmap v1.6.0
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
error[E0599]: no method named `name` found for struct `Id<'_>` in the current scope
   --> compiler/rustc_graphviz/src/tests.rs:114:41
    |
114 |             None => LabelStr(id_name(n).name()),
    |                                         ^^^^-- help: remove the arguments
    |                                         field, not a method
    | 
   ::: compiler/rustc_graphviz/src/lib.rs:381:1
    |
    |
381 | pub struct Id<'a> {
    | ----------------- method `name` not found for this
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_graphviz`
error: could not compile `rustc_graphviz`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_ast_pretty" "-p" "rustc_feature" "-p" "rustc_hir" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_fs_util" "-p" "rustc_apfloat" "-p" "rustc_index" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_symbol_mangling" "-p" "rustc_ast" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_data_structures" "-p" "rustc_session" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_driver" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_query_impl" "-p" "rustc_privacy" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_error_codes" "-p" "rustc_hir_pretty" "-p" "rustc_typeck" "-p" "rustc_mir_build" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:47
