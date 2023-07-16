plain
    Checking rustc_mir v0.0.0 (/checkout/compiler/rustc_mir)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_build/src/thir/mod.rs:431:17
    |
409 | pub enum InlineAsmOperand {
    |                          - help: consider introducing lifetime `'tcx` here: `<'tcx>`
...
431 |         value: &'tcx Const<'tcx>,
    |                 ^^^^ undeclared lifetime
    |
    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes

error[E0261]: use of undeclared lifetime name `'tcx`
   --> compiler/rustc_mir_build/src/thir/mod.rs:431:28
    |
409 | pub enum InlineAsmOperand {
    |                          - help: consider introducing lifetime `'tcx` here: `<'tcx>`
...
431 |         value: &'tcx Const<'tcx>,
    |                            ^^^^ undeclared lifetime
    |
    = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
error[E0308]: mismatched types
   --> compiler/rustc_mir_build/src/build/expr/into.rs:386:88
    |
    |
386 | ...                   mir::InlineAsmOperand::SymFn { value: box this.as_constant(expr) }
    |                                                                                  ^^^^ expected `&thir::Expr<'_>`, found struct `thir::ExprId`

error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'tcx` due to conflicting requirements
   --> compiler/rustc_mir_build/src/thir/cx/expr.rs:484:45
    |
484 | ...                   let value = ty::Const::from_anon_const(self.tcx, anon_const_def_id);
    |
    |
note: first, the lifetime cannot outlive the lifetime `'tcx` as defined on the impl at 20:6...
   --> compiler/rustc_mir_build/src/thir/cx/expr.rs:20:6
    |
20  | impl<'tcx> Cx<'tcx> {
    |      ^^^^
note: ...so that the expression is assignable
   --> compiler/rustc_mir_build/src/thir/cx/expr.rs:484:72
    |
484 | ...                   let value = ty::Const::from_anon_const(self.tcx, anon_const_def_id);
    |                                                              ^^^^^^^^
    = note: expected `TyCtxt<'_>`
               found `TyCtxt<'tcx>`
    = note: but, the lifetime must be valid for the static lifetime...
note: ...so that reference does not outlive borrowed content
   --> compiler/rustc_mir_build/src/thir/cx/expr.rs:487:59
487 | ...                   InlineAsmOperand::Const { value, span }
    |                                                 ^^^^^

error: aborting due to 4 previous errors
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_codegen_ssa" "-p" "rustc_symbol_mangling" "-p" "rustc_incremental" "-p" "rustc_graphviz" "-p" "rustc_apfloat" "-p" "rustc_hir" "-p" "rustc_feature" "-p" "rustc_index" "-p" "rustc_target" "-p" "rustc_fs_util" "-p" "rustc_serialize" "-p" "rustc_data_structures" "-p" "rustc_macros" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_arena" "-p" "rustc_query_system" "-p" "rustc_span" "-p" "rustc_session" "-p" "rustc_lint_defs" "-p" "rustc_errors" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_lexer" "-p" "rustc_ast" "-p" "rustc_driver" "-p" "rustc_interface" "-p" "rustc_ast_passes" "-p" "rustc_passes" "-p" "rustc_query_impl" "-p" "rustc_resolve" "-p" "rustc_privacy" "-p" "rustc_builtin_macros" "-p" "rustc_parse_format" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_ast_lowering" "-p" "rustc_ty_utils" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_expand" "-p" "rustc_metadata" "-p" "rustc_hir_pretty" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_typeck" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_error_codes" "-p" "rustc_parse" "-p" "rustc_save_analysis" "-p" "rustc_mir_build" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:26
