plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error[E0277]: the `?` operator can only be used in a closure that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
   --> compiler/rustc_typeck/src/check/mod.rs:398:41
    |
390 |                           let new_substs = InternalSubsts::for_item(self.tcx, def_id, |param, _| {
    |  _____________________________________________________________________________________-
391 | |                             let old_param = substs[param.index as usize];
392 | |                             match old_param.unpack() {
393 | |                                 GenericArgKind::Type(old_ty) => {
...   |
398 | |                                         old_param.fold_with(self)?
    | |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a closure that returns `rustc_middle::ty::subst::GenericArg<'_>`
426 | |                             }
427 | |                         });
427 | |                         });
    | |_________________________- this function should return `Result` or `Option` to accept `?`
    |
    = help: the trait `std::ops::Try` is not implemented for `rustc_middle::ty::subst::GenericArg<'_>`
    = note: required by `from_error`

error[E0277]: the `?` operator can only be used in a closure that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
   --> compiler/rustc_typeck/src/check/mod.rs:416:41
    |
390 |                           let new_substs = InternalSubsts::for_item(self.tcx, def_id, |param, _| {
    |  _____________________________________________________________________________________-
391 | |                             let old_param = substs[param.index as usize];
392 | |                             match old_param.unpack() {
393 | |                                 GenericArgKind::Type(old_ty) => {
...   |
416 | |                                         old_param.fold_with(self)?
    | |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a closure that returns `rustc_middle::ty::subst::GenericArg<'_>`
426 | |                             }
427 | |                         });
427 | |                         });
    | |_________________________- this function should return `Result` or `Option` to accept `?`
    |
    = help: the trait `std::ops::Try` is not implemented for `rustc_middle::ty::subst::GenericArg<'_>`
    = note: required by `from_error`

error[E0277]: the `?` operator can only be used in a closure that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
   --> compiler/rustc_typeck/src/check/mod.rs:423:41
    |
390 |                           let new_substs = InternalSubsts::for_item(self.tcx, def_id, |param, _| {
    |  _____________________________________________________________________________________-
391 | |                             let old_param = substs[param.index as usize];
392 | |                             match old_param.unpack() {
393 | |                                 GenericArgKind::Type(old_ty) => {
...   |
423 | |                                         old_param.fold_with(self)?
    | |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a closure that returns `rustc_middle::ty::subst::GenericArg<'_>`
426 | |                             }
427 | |                         });
427 | |                         });
    | |_________________________- this function should return `Result` or `Option` to accept `?`
    |
    = help: the trait `std::ops::Try` is not implemented for `rustc_middle::ty::subst::GenericArg<'_>`
    = note: required by `from_error`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_typeck`
error: could not compile `rustc_typeck`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_ast_pretty" "-p" "rustc_data_structures" "-p" "rustc_macros" "-p" "rustc_graphviz" "-p" "rustc_index" "-p" "rustc_ast" "-p" "rustc_lexer" "-p" "rustc_mir_build" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_attr" "-p" "rustc_apfloat" "-p" "rustc_infer" "-p" "rustc_arena" "-p" "rustc_target" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_fs_util" "-p" "rustc_typeck" "-p" "rustc_ty_utils" "-p" "rustc_hir" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_span" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_serialize" "-p" "rustc_hir_pretty" "-p" "rustc_errors" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_save_analysis" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_feature" "-p" "rustc_interface" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_incremental" "-p" "rustc_privacy" "-p" "rustc_symbol_mangling" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_query_impl" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_error_codes" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:25
