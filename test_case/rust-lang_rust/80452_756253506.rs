plain
    Checking chalk-solve v0.36.0
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
error[E0405]: cannot find trait `QueryContext` in this scope
   --> compiler/rustc_query_system/src/query/job.rs:430:10
    |
430 |     CTX: QueryContext,
    | 
    | 
   ::: compiler/rustc_query_system/src/dep_graph/mod.rs:22:1
    |
22  | pub trait DepContext: Copy {
    | -------------------------- similarly named trait `DepContext` defined here
help: a trait with a similar name exists
    |
    |
430 |     CTX: DepContext,
help: consider importing this trait
    |
    |
1   | use crate::query::QueryContext;


error[E0405]: cannot find trait `QueryContext` in this scope
   --> compiler/rustc_query_system/src/query/job.rs:457:22
    |
457 | fn remove_cycle<CTX: QueryContext>(
    | 
    | 
   ::: compiler/rustc_query_system/src/dep_graph/mod.rs:22:1
    |
22  | pub trait DepContext: Copy {
    | -------------------------- similarly named trait `DepContext` defined here
help: a trait with a similar name exists
    |
    |
457 | fn remove_cycle<CTX: DepContext>(
help: consider importing this trait
    |
    |
1   | use crate::query::QueryContext;


error[E0405]: cannot find trait `QueryContext` in this scope
   --> compiler/rustc_query_system/src/query/job.rs:563:22
    |
563 | pub fn deadlock<CTX: QueryContext>(tcx: CTX, registry: &rayon_core::Registry) {
    | 
    | 
   ::: compiler/rustc_query_system/src/dep_graph/mod.rs:22:1
    |
22  | pub trait DepContext: Copy {
    | -------------------------- similarly named trait `DepContext` defined here
help: a trait with a similar name exists
    |
    |
563 | pub fn deadlock<CTX: DepContext>(tcx: CTX, registry: &rayon_core::Registry) {
help: consider importing this trait
    |
    |
1   | use crate::query::QueryContext;

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0405`.
For more information about this error, try `rustc --explain E0405`.
error: could not compile `rustc_query_system`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_index" "-p" "rustc_apfloat" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_fs_util" "-p" "rustc_macros" "-p" "rustc_middle" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_type_ir" "-p" "rustc_hir" "-p" "rustc_serialize" "-p" "rustc_incremental" "-p" "rustc_span" "-p" "rustc_target" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_ast_pretty" "-p" "rustc_symbol_mangling" "-p" "rustc_errors" "-p" "rustc_ast" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_hir_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_interface" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_builtin_macros" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_typeck" "-p" "rustc_passes" "-p" "rustc_mir_build" "-p" "rustc_resolve" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_error_codes" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:50
