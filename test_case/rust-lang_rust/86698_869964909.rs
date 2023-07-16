plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `(dyn OnDiskCache<'tcx> + 'tcx)` cannot be shared between threads safely
    --> compiler/rustc_middle/src/ty/mod.rs:1614:14
     |
1614 |             .for_each(|&body_id| f(self.hir().body_owner_def_id(body_id)));
     |              ^^^^^^^^ --------------------------------------------------- within this `[closure@compiler/rustc_middle/src/ty/mod.rs:1614:23: 1614:74]`
     |              |
     |              `(dyn OnDiskCache<'tcx> + 'tcx)` cannot be shared between threads safely
     |
     = help: within `[closure@compiler/rustc_middle/src/ty/mod.rs:1614:23: 1614:74]`, the trait `std::marker::Sync` is not implemented for `(dyn OnDiskCache<'tcx> + 'tcx)`
     = note: required because it appears within the type `&'tcx (dyn OnDiskCache<'tcx> + 'tcx)`
     = note: required because it appears within the type `Option<&'tcx (dyn OnDiskCache<'tcx> + 'tcx)>`
note: required because it appears within the type `GlobalCtxt<'tcx>`
    --> compiler/rustc_middle/src/ty/context.rs:989:12
989  | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&'tcx GlobalCtxt<'tcx>`
note: required because it appears within the type `context::TyCtxt<'tcx>`
    --> compiler/rustc_middle/src/ty/context.rs:977:12
977  | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     = note: required because it appears within the type `&context::TyCtxt<'tcx>`
     = note: required because it appears within the type `&context::TyCtxt<'tcx>`
     = note: required because it appears within the type `[closure@compiler/rustc_middle/src/ty/mod.rs:1614:23: 1614:74]`

error[E0277]: `dyn OnDiskCache<'_>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/ty/context.rs:1791:13
     |
1791 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `dyn OnDiskCache<'_>` cannot be shared between threads safely
    ::: /checkout/compiler/rustc_data_structures/src/sync.rs:362:32
     |
     |
362  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ---- required by this bound in `assert_sync`
     |
     = help: within `ImplicitCtxt<'_, '_>`, the trait `std::marker::Sync` is not implemented for `dyn OnDiskCache<'_>`
     = note: required because it appears within the type `&dyn OnDiskCache<'_>`
     = note: required because it appears within the type `Option<&dyn OnDiskCache<'_>>`
note: required because it appears within the type `GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:989:12
989  | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
note: required because it appears within the type `context::TyCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:977:12
977  | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
note: required because it appears within the type `ImplicitCtxt<'_, '_>`
    --> compiler/rustc_middle/src/ty/context.rs:1701:16
     |
1701 |     pub struct ImplicitCtxt<'a, 'tcx> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_mir_build" "-p" "rustc_arena" "-p" "rustc_attr" "-p" "rustc_macros" "-p" "rustc_lexer" "-p" "rustc_apfloat" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_ast_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_lint_defs" "-p" "rustc_parse" "-p" "rustc_errors" "-p" "rustc_feature" "-p" "rustc_lint" "-p" "rustc_save_analysis" "-p" "rustc_data_structures" "-p" "rustc_serialize" "-p" "rustc_span" "-p" "rustc_ast" "-p" "rustc_plugin_impl" "-p" "rustc_hir" "-p" "rustc_interface" "-p" "rustc_query_impl" "-p" "rustc_traits" "-p" "rustc_incremental" "-p" "rustc_symbol_mangling" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_privacy" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_ast_passes" "-p" "rustc_ty_utils" "-p" "rustc_ast_lowering" "-p" "rustc_expand" "-p" "rustc_hir_pretty" "-p" "rustc_target" "-p" "rustc_typeck" "-p" "rustc_metadata" "-p" "rustc_error_codes" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:59
