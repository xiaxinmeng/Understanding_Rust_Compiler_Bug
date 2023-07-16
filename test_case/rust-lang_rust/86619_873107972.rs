plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `Cell<Option<EventId>>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/ty/mod.rs:1635:14
     |
1635 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
     |              ^^^^^^^^ --------------------------------------------------- within this `[closure@compiler/rustc_middle/src/ty/mod.rs:1635:23: 1635:74]`
     |              |
     |              `Cell<Option<EventId>>` cannot be shared between threads safely
     |
     = help: within `[closure@compiler/rustc_middle/src/ty/mod.rs:1635:23: 1635:74]`, the trait `std::marker::Sync` is not implemented for `Cell<Option<EventId>>`
     = note: required because it appears within the type `rustc_query_system::dep_graph::DepGraph<dep_node::DepKind>`
note: required because it appears within the type `GlobalCtxt<'tcx>`
    --> compiler/rustc_middle/src/ty/context.rs:963:12
963  | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&'tcx GlobalCtxt<'tcx>`
note: required because it appears within the type `context::TyCtxt<'tcx>`
    --> compiler/rustc_middle/src/ty/context.rs:951:12
951  | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     = note: required because it appears within the type `&context::TyCtxt<'tcx>`
     = note: required because it appears within the type `&context::TyCtxt<'tcx>`
     = note: required because it appears within the type `[closure@compiler/rustc_middle/src/ty/mod.rs:1635:23: 1635:74]`

error[E0277]: `Cell<Option<EventId>>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/ty/context.rs:1759:13
     |
1759 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Cell<Option<EventId>>` cannot be shared between threads safely
    ::: /checkout/compiler/rustc_data_structures/src/sync.rs:362:32
     |
     |
362  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ---- required by this bound in `assert_sync`
     |
     = help: within `ImplicitCtxt<'_, '_>`, the trait `std::marker::Sync` is not implemented for `Cell<Option<EventId>>`
     = note: required because it appears within the type `rustc_query_system::dep_graph::DepGraph<dep_node::DepKind>`
note: required because it appears within the type `GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:963:12
963  | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
note: required because it appears within the type `context::TyCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:951:12
951  | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
note: required because it appears within the type `ImplicitCtxt<'_, '_>`
    --> compiler/rustc_middle/src/ty/context.rs:1669:16
     |
1669 |     pub struct ImplicitCtxt<'a, 'tcx> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_data_structures" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_graphviz" "-p" "rustc_plugin_impl" "-p" "rustc_interface" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_resolve" "-p" "rustc_ast_passes" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_infer" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_incremental" "-p" "rustc_ty_utils" "-p" "rustc_mir_build" "-p" "rustc_apfloat" "-p" "rustc_attr" "-p" "rustc_passes" "-p" "rustc_typeck" "-p" "rustc_traits" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_parse" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_hir" "-p" "rustc_ast_pretty" "-p" "rustc_session" "-p" "rustc_span" "-p" "rustc_metadata" "-p" "rustc_ast" "-p" "rustc_serialize" "-p" "rustc_hir_pretty" "-p" "rustc_save_analysis" "-p" "rustc_feature" "-p" "rustc_target" "-p" "rustc_errors" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:11
