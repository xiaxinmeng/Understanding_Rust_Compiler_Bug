plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0583]: file not found for module `track_path`
  --> compiler/rustc_builtin_macros/src/lib.rs:45:1
   |
45 | mod track_path;
   |
   |
   = help: to create the module `track_path`, create file "compiler/rustc_builtin_macros/src/track_path.rs"

error[E0425]: cannot find value `expand_track_path` in module `track_path`
  --> compiler/rustc_builtin_macros/src/lib.rs:90:33
   |
90 |         track_path: track_path::expand_track_path,
   |                                 ^^^^^^^^^^^^^^^^^ not found in `track_path`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0583.
For more information about an error, try `rustc --explain E0425`.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustc_builtin_macros`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_symbol_mangling" "-p" "rustc_session" "-p" "rustc_feature" "-p" "rustc_lint_defs" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_hir" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_serialize" "-p" "rustc_index" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_macros" "-p" "rustc_errors" "-p" "rustc_span" "-p" "rustc_target" "-p" "rustc_ast" "-p" "rustc_fs_util" "-p" "rustc_apfloat" "-p" "rustc_incremental" "-p" "rustc_driver" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "-p" "rustc_interface" "-p" "rustc_ty_utils" "-p" "rustc_infer" "-p" "rustc_ast_passes" "-p" "rustc_ast_lowering" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_expand" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_query_impl" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_traits" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_lint" "-p" "rustc_typeck" "-p" "rustc_mir_build" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:06
