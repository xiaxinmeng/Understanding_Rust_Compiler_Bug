plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0308]: mismatched types
   --> compiler/rustc_symbol_mangling/src/v0.rs:490:17
    |
489 |             match predicate {
    |                   --------- this expression has type `Binder<'_, ExistentialPredicate<'_>>`
490 |                 ty::ExistentialPredicate::Trait(trait_ref) => {
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Binder`, found enum `ExistentialPredicate`
    |
    = note: expected struct `Binder<'_, ExistentialPredicate<'_>, >`
                 found enum `ExistentialPredicate<'_>`
help: you might have meant to use field `0` whose type is `ExistentialPredicate<'_>`
489 |             match predicate.0 {
    |                   ^^^^^^^^^^^

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_symbol_mangling/src/v0.rs:498:17
    |
489 |             match predicate {
    |                   --------- this expression has type `Binder<'_, ExistentialPredicate<'_>>`
...
498 |                 ty::ExistentialPredicate::Projection(projection) => {
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Binder`, found enum `ExistentialPredicate`
    |
    = note: expected struct `Binder<'_, ExistentialPredicate<'_>, >`
                 found enum `ExistentialPredicate<'_>`
help: you might have meant to use field `0` whose type is `ExistentialPredicate<'_>`
489 |             match predicate.0 {
    |                   ^^^^^^^^^^^

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_symbol_mangling/src/v0.rs:504:17
    |
489 |             match predicate {
    |                   --------- this expression has type `Binder<'_, ExistentialPredicate<'_>>`
...
504 |                 ty::ExistentialPredicate::AutoTrait(def_id) => {
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Binder`, found enum `ExistentialPredicate`
    |
    = note: expected struct `Binder<'_, ExistentialPredicate<'_>, >`
                 found enum `ExistentialPredicate<'_>`
help: you might have meant to use field `0` whose type is `ExistentialPredicate<'_>`
489 |             match predicate.0 {
    |                   ^^^^^^^^^^^

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_symbol_mangling/src/v0.rs:491:76
    |
491 |                       self = self.in_binder(&predicate, |mut cx, _predicate| {
    |  ____________________________________________________________________________^
492 | |                         // Use a type that can't appear in defaults of type parameters.
493 | |                         let dummy_self = cx.tcx.mk_ty_infer(ty::FreshTy(0));
494 | |                         let trait_ref = trait_ref.with_self_ty(cx.tcx, dummy_self);
495 | |                         cx = cx.print_def_path(trait_ref.def_id, trait_ref.substs)?;
    | |_____________________^ expected enum `std::result::Result`, found `()`
    |
    |
    = note:   expected enum `std::result::Result<SymbolMangler<'_>, !>`

error[E0308]: mismatched types
   --> compiler/rustc_symbol_mangling/src/v0.rs:491:28
    |
    |
491 |                       self = self.in_binder(&predicate, |mut cx, _predicate| {
    |  ____________________________^
492 | |                         // Use a type that can't appear in defaults of type parameters.
493 | |                         let dummy_self = cx.tcx.mk_ty_infer(ty::FreshTy(0));
494 | |                         let trait_ref = trait_ref.with_self_ty(cx.tcx, dummy_self);
495 | |                         cx = cx.print_def_path(trait_ref.def_id, trait_ref.substs)?;
496 | |                     })
    | |______________________^ expected struct `SymbolMangler`, found enum `std::result::Result`
    |
    = note: expected struct `SymbolMangler<'tcx>`
                 found enum `std::result::Result<SymbolMangler<'_>, !>`

error[E0614]: type `DefId` cannot be dereferenced
   --> compiler/rustc_symbol_mangling/src/v0.rs:505:48
    |
505 |                     self = self.print_def_path(*def_id, &[])?;

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0308, E0614.
Some errors have detailed explanations: E0308, E0614.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_symbol_mangling`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_symbol_mangling" "-p" "rustc_serialize" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_feature" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_hir" "-p" "rustc_index" "-p" "rustc_target" "-p" "rustc_errors" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_apfloat" "-p" "rustc_macros" "-p" "rustc_data_structures" "-p" "rustc_fs_util" "-p" "rustc_ast" "-p" "rustc_driver" "-p" "rustc_error_codes" "-p" "rustc_interface" "-p" "rustc_ast_lowering" "-p" "rustc_resolve" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_expand" "-p" "rustc_query_impl" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_ty_utils" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_ast_passes" "-p" "rustc_privacy" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_save_analysis" "-p" "rustc_lint" "-p" "rustc_mir_build" "-p" "rustc_typeck" "-p" "rustc_plugin_impl" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_hir_pretty" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:45
