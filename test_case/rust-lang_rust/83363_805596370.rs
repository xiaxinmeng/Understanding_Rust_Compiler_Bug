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
error[E0599]: no method named `with_help` found for struct `proc_macro::Diagnostic` in the current scope
   --> compiler/rustc_macros/src/session_diagnostic.rs:115:19
    |
115 |               $diag.with_help($msg)
    |                     ^^^^^^^^^ method not found in `proc_macro::Diagnostic`
...
493 |                                       |diag| with_help!(
494 | |                                         diag,
494 | |                                         diag,
495 | |                                         "#[suggestion(...)] on a tuple field must be applied to fields of type (Span, Applicability)"
    | |_____________________________________- in this macro invocation
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0599]: no method named `with_help` found for struct `proc_macro::Diagnostic` in the current scope
   --> compiler/rustc_macros/src/session_diagnostic.rs:115:19
    |
115 |               $diag.with_help($msg)
    |                     ^^^^^^^^^ method not found in `proc_macro::Diagnostic`
...
502 |                                   |diag| with_help!(
503 | |                                     diag,
503 | |                                     diag,
504 | |                                     "#[suggestion(...)] should be applied to fields of type Span or (Span, Applicability)"
    | |_________________________________- in this macro invocation
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0599]: no method named `with_help` found for struct `proc_macro::Diagnostic` in the current scope
   --> compiler/rustc_macros/src/session_diagnostic.rs:115:19
    |
115 |               $diag.with_help($msg)
    |                     ^^^^^^^^^ method not found in `proc_macro::Diagnostic`
...
551 |                                   |diag| with_help!(
552 | |                                     diag,
552 | |                                     diag,
553 | |                                     "provide a suggestion message using #[suggestion(message = \"...\")]"
    | |_________________________________- in this macro invocation
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0599]: no method named `with_help` found for struct `proc_macro::Diagnostic` in the current scope
   --> compiler/rustc_macros/src/session_diagnostic.rs:115:19
    |
115 |               $diag.with_help($msg)
    |                     ^^^^^^^^^ method not found in `proc_macro::Diagnostic`
205 | /                         with_help!(
205 | /                         with_help!(
206 | |                             ast.span().unwrap().error("`code` not specified"),
207 | |                             "use the [code = \"...\"] attribute to set this diagnostic's error code"
    | |_________________________- in this macro invocation
    |
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_symbol_mangling" "-p" "rustc_target" "-p" "rustc_incremental" "-p" "rustc_macros" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_errors" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_hir" "-p" "rustc_ast" "-p" "rustc_driver" "-p" "rustc_typeck" "-p" "rustc_infer" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_hir_pretty" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_plugin_impl" "-p" "rustc_mir_build" "-p" "rustc_save_analysis" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_query_impl" "-p" "rustc_ast_lowering" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:50
