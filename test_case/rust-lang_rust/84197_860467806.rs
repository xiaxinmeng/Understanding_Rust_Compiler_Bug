plain
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error[E0308]: mismatched types
   --> /checkout/library/core/src/macros/mod.rs:88:33
    |
84  | / macro_rules! assert_ne {
85  | |     ($left:expr, $right:expr $(,)?) => ({
86  | |         match (&$left, &$right) {
87  | |             (left_val, right_val) => {
88  | |                 if *left_val == *right_val {
    | |                                 ^^^^^^^^^^ expected enum `StackProtector`, found enum `Option`
110 | |     });
111 | | }
111 | | }
    | |_- in this expansion of `assert_ne!` (#2)
   ::: compiler/rustc_interface/src/tests.rs:677:5
    |
677 | /     macro_rules! tracked {
677 | /     macro_rules! tracked {
678 | |         ($name: ident, $non_default_value: expr) => {
679 | |             opts = reference.clone();
680 | |             assert_ne!(opts.debugging_opts.$name, $non_default_value);
...   |
683 | |         };
684 | |     }
684 | |     }
    | |_____- in this expansion of `tracked!` (#1)
...
740 |       tracked!(stack_protector, Some(StackProtector::All));
    |
    = note: expected enum `StackProtector`
               found enum `Option<StackProtector>`


error[E0308]: mismatched types
   --> compiler/rustc_interface/src/tests.rs:740:31
    |
740 |     tracked!(stack_protector, Some(StackProtector::All));
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `StackProtector`, found enum `Option`
    = note: expected enum `StackProtector`
               found enum `Option<StackProtector>`

error: aborting due to 2 previous errors
---
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_driver test:false 0.576
[RUSTC-TIMING] rustc_driver test:true 0.602
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "--all-targets" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_ast_pretty" "-p" "rustc_ast" "-p" "rustc_macros" "-p" "rustc_lexer" "-p" "rustc_index" "-p" "rustc_interface" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_parse_format" "-p" "rustc_ast_passes" "-p" "rustc_traits" "-p" "rustc_attr" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_arena" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_fs_util" "-p" "rustc_ty_utils" "-p" "rustc_incremental" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_parse" "-p" "rustc_errors" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_lint" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_session" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_mir_build" "-p" "rustc_typeck" "-p" "rustc_plugin_impl" "-p" "rustc_metadata" "-p" "rustc_save_analysis" "-p" "rustc_data_structures" "-p" "rustc_serialize" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:01:36
