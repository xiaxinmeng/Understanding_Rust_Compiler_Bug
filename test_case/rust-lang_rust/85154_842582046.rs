plain
    Checking rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
    Checking rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0407]: method `query_crate` is not a member of trait `Key`
  --> compiler/rustc_query_impl/src/keys.rs:25:5
25 | /     fn query_crate(&self) -> CrateNum {
26 | |         LOCAL_CRATE
27 | |     }
   | |_____^ not a member of trait `Key`
   | |_____^ not a member of trait `Key`

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0046]: not all trait items implemented, missing: `query_crate_is_local`
  --> compiler/rustc_query_impl/src/keys.rs:24:1
   |
17 |     fn query_crate_is_local(&self) -> bool;
   |     --------------------------------------- `query_crate_is_local` from trait
...
24 | impl Key for () {
   | ^^^^^^^^^^^^^^^ missing `query_crate_is_local` in implementation
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0046, E0407.
Some errors have detailed explanations: E0046, E0407.
For more information about an error, try `rustc --explain E0046`.
error: could not compile `rustc_query_impl`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_symbol_mangling" "-p" "rustc_macros" "-p" "rustc_fs_util" "-p" "rustc_incremental" "-p" "rustc_target" "-p" "rustc_serialize" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_apfloat" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_index" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_errors" "-p" "rustc_driver" "-p" "rustc_mir_build" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_hir_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_query_impl" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_lint" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:13
