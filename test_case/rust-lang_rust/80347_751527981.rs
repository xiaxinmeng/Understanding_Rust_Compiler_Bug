plain
    Checking rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
    Checking rustc_metadata v0.0.0 (/checkout/compiler/rustc_metadata)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0599]: no method named `for_each` found for struct `rustc_rayon::collections::hash_set::Iter<'_, LocalDefId>` in the current scope
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1782:41
     |
1782 |     par_iter(tcx.mir_keys(LOCAL_CRATE)).for_each(|&def_id| {
     |                                         ^^^^^^^^ method not found in `rustc_rayon::collections::hash_set::Iter<'_, LocalDefId>`
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.3.0/src/collections/hash_set.rs:31:1
     |
     |
31   | pub struct Iter<'a, T: Hash + Eq + Sync + 'a> {
     | --------------------------------------------- doesn't satisfy `_: Iterator`
     |
     = note: the method `for_each` exists but the following trait bounds were not satisfied:
             `rustc_rayon::collections::hash_set::Iter<'_, LocalDefId>: Iterator`
             which is required by `&mut rustc_rayon::collections::hash_set::Iter<'_, LocalDefId>: Iterator`
     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
     = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
             `use crate::rustc_data_structures::sync::ParallelIterator;`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_metadata`
error: could not compile `rustc_metadata`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_serialize" "-p" "rustc_macros" "-p" "rustc_middle" "-p" "rustc_arena" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_query_system" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_type_ir" "-p" "rustc_interface" "-p" "rustc_passes" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_fs_util" "-p" "rustc_resolve" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_ast_passes" "-p" "rustc_expand" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_infer" "-p" "rustc_privacy" "-p" "rustc_mir_build" "-p" "rustc_symbol_mangling" "-p" "rustc_traits" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_ast_lowering" "-p" "rustc_builtin_macros" "-p" "rustc_lint" "-p" "rustc_ast" "-p" "rustc_target" "-p" "rustc_span" "-p" "rustc_error_codes" "-p" "rustc_ast_pretty" "-p" "rustc_parse" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_save_analysis" "-p" "rustc_data_structures" "-p" "rustc_errors" "-p" "rustc_hir" "-p" "rustc_plugin_impl" "-p" "rustc_metadata" "-p" "rustc_feature" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_hir_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:24
