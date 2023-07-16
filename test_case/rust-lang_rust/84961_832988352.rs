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
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error: unused import: `edition`
 --> compiler/rustc_span/src/symbol/tests.rs:3:50
  |
3 | use crate::{create_default_session_globals_then, edition};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> compiler/rustc_span/src/lev_distance/tests.rs:25:5
    |
    |
25  |       create_session_globals_then(|| {
    |  _____^^^^^^^^^^^^^^^^^^^^^^^^^^^_-
    | |     expected 2 arguments
    | |     expected 2 arguments
26  | |         let input = vec![Symbol::intern("aaab"), Symbol::intern("aaabc")];
27  | |         assert_eq!(
28  | |             find_best_match_for_name(&input, Symbol::intern("aaaa"), None),
54  | |         );
55  | |     })
    | |_____- supplied 1 argument
    |
    |
note: function defined here
   --> compiler/rustc_span/src/lib.rs:103:8
    |
103 | pub fn create_session_globals_then<R>(edition: Edition, f: impl FnOnce() -> R) -> R {
    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^    ----------------  ---------------------
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustc_span`
error: could not compile `rustc_span`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_feature" "-p" "rustc_parse" "-p" "rustc_lexer" "-p" "rustc_interface" "-p" "rustc_expand" "-p" "rustc_macros" "-p" "rustc_lint_defs" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_traits" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_ast_passes" "-p" "rustc_attr" "-p" "rustc_symbol_mangling" "-p" "rustc_ty_utils" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_index" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_passes" "-p" "rustc_hir" "-p" "rustc_target" "-p" "rustc_data_structures" "-p" "rustc_ast_pretty" "-p" "rustc_serialize" "-p" "rustc_mir" "-p" "rustc_apfloat" "-p" "coverage_test_macros" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_metadata" "-p" "rustc_session" "-p" "rustc_hir_pretty" "-p" "rustc_mir_build" "-p" "rustc_ast" "-p" "rustc_errors" "-p" "rustc_typeck" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_span" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:05
