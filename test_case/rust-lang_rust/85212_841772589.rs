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
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking chalk-engine v0.55.0
error[E0560]: struct `TokenCursor<true>` has no field named `num_next_calls`
   |
   |
17 |         num_next_calls: 0,
   |         ^^^^^^^^^^^^^^ `TokenCursor<{_: bool}>` does not have this field
   |
   = note: available fields are: `frame`, `stack`, `nncablt`

error[E0560]: struct `TokenCursor<true>` has no field named `break_last_token`
   |
   |
18 |         break_last_token: true,
   |         ^^^^^^^^^^^^^^^^ `TokenCursor<{_: bool}>` does not have this field
   |
   = note: available fields are: `frame`, `stack`, `nncablt`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0560`.
error: could not compile `rustc_parse`
error: could not compile `rustc_parse`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_target" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_serialize" "-p" "rustc_mir" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_graphviz" "-p" "rustc_attr" "-p" "coverage_test_macros" "-p" "rustc_lexer" "-p" "rustc_infer" "-p" "rustc_apfloat" "-p" "rustc_data_structures" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_hir_pretty" "-p" "rustc_mir_build" "-p" "rustc_arena" "-p" "rustc_span" "-p" "rustc_plugin_impl" "-p" "rustc_errors" "-p" "rustc_feature" "-p" "rustc_ast" "-p" "rustc_ast_pretty" "-p" "rustc_parse" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_save_analysis" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_error_codes" "-p" "rustc_hir" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_lint" "-p" "rustc_interface" "-p" "rustc_incremental" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_symbol_mangling" "-p" "rustc_query_impl" "-p" "rustc_passes" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:17
