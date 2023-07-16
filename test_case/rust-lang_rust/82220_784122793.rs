plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
error[E0658]: `if let` guards are experimental
   --> compiler/rustc_typeck/src/check/callee.rs:388:60
    |
388 | ...                   Res::Def(kind, def_id) if let Some(Namespace::ValueNS) = kind.ns() => {
    |
    = note: see issue #51114 <https://github.com/rust-lang/rust/issues/51114> for more information
    = note: see issue #51114 <https://github.com/rust-lang/rust/issues/51114> for more information
    = help: add `#![feature(if_let_guard)]` to the crate attributes to enable
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_macros" "-p" "rustc_index" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_apfloat" "-p" "rustc_query_system" "-p" "rustc_type_ir" "-p" "rustc_hir" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ast_passes" "-p" "rustc_error_codes" "-p" "rustc_ast" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_typeck" "-p" "rustc_infer" "-p" "rustc_ty_utils" "-p" "rustc_privacy" "-p" "rustc_mir_build" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_symbol_mangling" "-p" "rustc_query_impl" "-p" "rustc_traits" "-p" "rustc_trait_selection" "-p" "rustc_session" "-p" "rustc_serialize" "-p" "rustc_feature" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_ast_pretty" "-p" "rustc_errors" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_parse" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:35
