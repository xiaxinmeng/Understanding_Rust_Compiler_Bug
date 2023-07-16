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
    Checking parking_lot v0.11.0
    Checking measureme v9.1.0
    Checking synstructure v0.12.4
    Checking rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
error[E0405]: cannot find trait `Spanned` in crate `proc_macro`
   --> compiler/rustc_macros/src/session_diagnostic.rs:114:36
    |
114 | fn span_err(span: impl proc_macro::Spanned, msg: &str) -> proc_macro::Diagnostic {
    |                                    ^^^^^^^ not found in `proc_macro`
help: consider importing one of these items
    |
    |
2   | use crate::query::Spanned;
    |
2   | use quote::spanned::Spanned;
    |
2   | use syn::spanned::Spanned;


error[E0405]: cannot find trait `Spanned` in crate `proc_macro`
   --> compiler/rustc_macros/src/session_diagnostic.rs:132:51
    |
132 |     #[cfg(not(bootstrap))] span: impl proc_macro::Spanned,
    |                                                   ^^^^^^^ not found in `proc_macro`
help: consider importing one of these items
    |
    |
2   | use crate::query::Spanned;
    |
2   | use quote::spanned::Spanned;
    |
2   | use syn::spanned::Spanned;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0405`.
For more information about this error, try `rustc --explain E0405`.
error: could not compile `rustc_macros`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_mir" "-p" "rustc_infer" "-p" "rustc_lexer" "-p" "rustc_attr" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_apfloat" "-p" "rustc_graphviz" "-p" "coverage_test_macros" "-p" "rustc_parse" "-p" "rustc_ast" "-p" "rustc_typeck" "-p" "rustc_arena" "-p" "rustc_hir" "-p" "rustc_save_analysis" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_lint" "-p" "rustc_hir_pretty" "-p" "rustc_mir_build" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_symbol_mangling" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_incremental" "-p" "rustc_ty_utils" "-p" "rustc_ast_lowering" "-p" "rustc_feature" "-p" "rustc_errors" "-p" "rustc_session" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_data_structures" "-p" "rustc_error_codes" "-p" "rustc_serialize" "-p" "rustc_span" "-p" "rustc_plugin_impl" "-p" "rustc_ast_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:54
