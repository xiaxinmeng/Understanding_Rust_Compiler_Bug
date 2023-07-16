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
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0277]: can't compare `bool` with `Option<bool>`
    |
84  | / macro_rules! assert_ne {
84  | / macro_rules! assert_ne {
85  | |     ($left:expr, $right:expr $(,)?) => ({
86  | |         match (&$left, &$right) {
87  | |             (left_val, right_val) => {
88  | |                 if *left_val == *right_val {
    | |                              ^^ no implementation for `bool == Option<bool>`
110 | |     });
111 | | }
111 | | }
    | |_- in this expansion of `assert_ne!` (#2)
   ::: compiler/rustc_interface/src/tests.rs:673:5
    |
673 | /     macro_rules! tracked {
673 | /     macro_rules! tracked {
674 | |         ($name: ident, $non_default_value: expr) => {
675 | |             opts = reference.clone();
676 | |             assert_ne!(opts.debugging_opts.$name, $non_default_value);
    | |             ---------------------------------------------------------- in this macro invocation (#2)
679 | |         };
680 | |     }
680 | |     }
    | |_____- in this expansion of `tracked!` (#1)
...
709 |       tracked!(metadata_link, Some(true));
    |       ------------------------------------ in this macro invocation (#1)
    |
    = help: the trait `std::cmp::PartialEq<Option<bool>>` is not implemented for `bool`
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:709:29
    |
    |
709 |     tracked!(metadata_link, Some(true));
    |                             ^^^^^^^^^^ expected `bool`, found enum `Option`
    = note: expected type `bool`
               found enum `Option<bool>`

    Checking rustc-main v0.0.0 (/checkout/compiler/rustc)
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_infer" "-p" "rustc_macros" "-p" "rustc_graphviz" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_arena" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_errors" "-p" "rustc_feature" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_typeck" "-p" "rustc_lint" "-p" "rustc_session" "-p" "rustc_target" "-p" "rustc_hir_pretty" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_save_analysis" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_ast" "-p" "rustc_parse" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_data_structures" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_error_codes" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:33
