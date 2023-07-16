plain
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error[E0658]: use of unstable library feature 'iter_zip'
   --> compiler/rustc_query_system/src/query/job.rs:451:33
    |
451 |         let mut stack: Vec<_> = iter::zip(spans, queries).collect();
    |
    = note: see issue #83574 <https://github.com/rust-lang/rust/issues/83574> for more information
    = note: see issue #83574 <https://github.com/rust-lang/rust/issues/83574> for more information
    = help: add `#![feature(iter_zip)]` to the crate attributes to enable
    Checking chalk-engine v0.55.0
error[E0658]: use of unstable library feature 'bool_to_option'
error[E0658]: use of unstable library feature 'bool_to_option'
   --> compiler/rustc_query_system/src/query/job.rs:398:58
    |
398 |         connected_to_root(query_map, successor, visited).then_some(None)
    |
    = note: see issue #64260 <https://github.com/rust-lang/rust/issues/64260> for more information
    = note: see issue #64260 <https://github.com/rust-lang/rust/issues/64260> for more information
    = help: add `#![feature(bool_to_option)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
error: could not compile `rustc_query_system`
error: could not compile `rustc_query_system`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_serialize" "-p" "rustc_typeck" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_arena" "-p" "rustc_ty_utils" "-p" "rustc_attr" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_ast" "-p" "rustc_errors" "-p" "rustc_lint_defs" "-p" "rustc_data_structures" "-p" "rustc_error_codes" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_parse" "-p" "rustc_hir" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint" "-p" "rustc_ast_pretty" "-p" "rustc_feature" "-p" "rustc_plugin_impl" "-p" "rustc_save_analysis" "-p" "rustc_span" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_resolve" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_passes" "-p" "rustc_privacy" "-p" "rustc_incremental" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_hir_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:45
