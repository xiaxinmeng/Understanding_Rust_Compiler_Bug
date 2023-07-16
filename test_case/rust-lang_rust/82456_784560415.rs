plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
    --> compiler/rustc_typeck/src/check/mod.rs:1068:14
     |
1068 |             .map_or_else(|| String::new(), |s| format!(" `{}`", s)),
     |              ^^^^^^^^^^^ -- takes 0 arguments
     |              expected closure that takes 1 argument
     |
help: consider changing the closure to take and ignore the expected argument
     |
     |
1068 |             .map_or_else(|_| String::new(), |s| format!(" `{}`", s)),

    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
   --> compiler/rustc_typeck/src/check/pat.rs:882:18
   --> compiler/rustc_typeck/src/check/pat.rs:882:18
    |
882 |                 .map_or_else(|| String::new(), |s| format!(" `{}`", s.trim_end()));
    |                  ^^^^^^^^^^^ -- takes 0 arguments
    |                  expected closure that takes 1 argument
    |
help: consider changing the closure to take and ignore the expected argument
    |
    |
882 |                 .map_or_else(|_| String::new(), |s| format!(" `{}`", s.trim_end()));

error[E0593]: closure is expected to take 1 argument, but it takes 0 arguments
    --> compiler/rustc_typeck/src/collect.rs:2390:22
     |
     |
2390 |                     .map_or_else(|| String::new(), |s| format!(" `{}`", s));
     |                      ^^^^^^^^^^^ -- takes 0 arguments
     |                      expected closure that takes 1 argument
     |
help: consider changing the closure to take and ignore the expected argument
     |
     |
2390 |                     .map_or_else(|_| String::new(), |s| format!(" `{}`", s));

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0593`.
For more information about this error, try `rustc --explain E0593`.
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_error_codes" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_hir" "-p" "rustc_ast_pretty" "-p" "rustc_target" "-p" "rustc_middle" "-p" "rustc_apfloat" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_lint_defs" "-p" "rustc_interface" "-p" "rustc_mir_build" "-p" "rustc_infer" "-p" "rustc_passes" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_typeck" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_symbol_mangling" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_query_impl" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_builtin_macros" "-p" "rustc_privacy" "-p" "rustc_incremental" "-p" "rustc_feature" "-p" "rustc_serialize" "-p" "rustc_parse" "-p" "rustc_ast" "-p" "rustc_lint" "-p" "rustc_errors" "-p" "rustc_save_analysis" "-p" "rustc_span" "-p" "rustc_session" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:25
