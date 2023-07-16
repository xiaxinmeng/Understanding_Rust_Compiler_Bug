plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0308]: mismatched types
  --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:58:46
   |
57 | ...                   match (inner_ty.kind(), target_ty.kind()) {
   |                             ----------------------------------- this expression has type `(&rustc_middle::ty::TyKind<'_>, &rustc_middle::ty::TyKind<'_>)`
58 | ...                       (Infer(TyVar(&a_vid)), Infer(TyVar(&b_vid))) => self
   |                                        |
   |                                        |
   |                                        expected struct `TyVid`, found reference
   |                                        help: you can probably remove the explicit borrow: `a_vid`
   = note: expected struct `TyVid`
           found reference `&_`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> compiler/rustc_infer/src/infer/error_reporting/need_type_info.rs:58:68
   |
57 | ...                   match (inner_ty.kind(), target_ty.kind()) {
   |                             ----------------------------------- this expression has type `(&rustc_middle::ty::TyKind<'_>, &rustc_middle::ty::TyKind<'_>)`
58 | ...                       (Infer(TyVar(&a_vid)), Infer(TyVar(&b_vid))) => self
   |                                                              |
   |                                                              |
   |                                                              expected struct `TyVid`, found reference
   |                                                              help: you can probably remove the explicit borrow: `b_vid`
   = note: expected struct `TyVid`
           found reference `&_`

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_infer`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_feature" "-p" "rustc_query_system" "-p" "rustc_arena" "-p" "rustc_index" "-p" "rustc_span" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_hir" "-p" "rustc_target" "-p" "rustc_macros" "-p" "rustc_errors" "-p" "rustc_attr" "-p" "rustc_lexer" "-p" "rustc_ast_pretty" "-p" "rustc_fs_util" "-p" "rustc_symbol_mangling" "-p" "rustc_apfloat" "-p" "rustc_data_structures" "-p" "rustc_serialize" "-p" "rustc_ast" "-p" "rustc_driver" "-p" "rustc_lint" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_parse" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_ast_passes" "-p" "rustc_error_codes" "-p" "rustc_save_analysis" "-p" "rustc_hir_pretty" "-p" "rustc_plugin_impl" "-p" "rustc_interface" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "rustc_traits" "-p" "rustc_builtin_macros" "-p" "rustc_ast_lowering" "-p" "rustc_mir_build" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_typeck" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:32
