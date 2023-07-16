plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
error: unused variable: `for_crate_hash`
   --> compiler/rustc_session/src/options.rs:257:37
    |
199 | / macro_rules! options {
200 | |     ($struct_name:ident, $setter_name:ident, $defaultfn:ident,
201 | |      $buildfn:ident, $prefix:expr, $outputname:expr,
202 | |      $stat:ident, $mod_desc:ident, $mod_set:ident,
...   |
257 | |         fn dep_tracking_hash(&self, for_crate_hash: bool, error_format: ErrorOutputType) -> u64 {
    | |                                     ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_for_crate_hash`
810 | |     }
811 | | ) }
811 | | ) }
    | |___- in this expansion of `options!`
812 | 
813 | / options! {CodegenOptions, CodegenSetter, basic_codegen_options,
814 | |           build_codegen_options, "C", "codegen",
815 | |           CG_OPTIONS, cg_type_desc, cgsetters,
...   |
920 | |     // - src/doc/rustc/src/codegen-options/index.md
921 | | }
    | |_- in this macro invocation
    | |_- in this macro invocation
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `for_crate_hash`
    --> compiler/rustc_session/src/options.rs:257:37
     |
199  | / macro_rules! options {
199  | / macro_rules! options {
200  | |     ($struct_name:ident, $setter_name:ident, $defaultfn:ident,
201  | |      $buildfn:ident, $prefix:expr, $outputname:expr,
202  | |      $stat:ident, $mod_desc:ident, $mod_set:ident,
...    |
257  | |         fn dep_tracking_hash(&self, for_crate_hash: bool, error_format: ErrorOutputType) -> u64 {
     | |                                     ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_for_crate_hash`
810  | |     }
811  | | ) }
811  | | ) }
     | |___- in this expansion of `options!`
...
923  | / options! {DebuggingOptions, DebuggingSetter, basic_debugging_options,
924  | |           build_debugging_options, "Z", "debugging",
925  | |           DB_OPTIONS, db_type_desc, dbsetters,
...    |
1257 | |     // - compiler/rustc_interface/src/tests.rs
1258 | | }
     | |_- in this macro invocation
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_metadata" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ast_passes" "-p" "rustc_typeck" "-p" "rustc_arena" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_span" "-p" "rustc_serialize" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_lint" "-p" "rustc_parse" "-p" "rustc_hir_pretty" "-p" "rustc_hir" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_symbol_mangling" "-p" "rustc_privacy" "-p" "rustc_query_impl" "-p" "rustc_incremental" "-p" "rustc_builtin_macros" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_feature" "-p" "rustc_data_structures" "-p" "rustc_ast_pretty" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_errors" "-p" "rustc_ast" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:19
