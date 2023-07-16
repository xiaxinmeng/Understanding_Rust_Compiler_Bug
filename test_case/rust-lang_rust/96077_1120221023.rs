plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: unused import: `rustc_middle::ty`
 --> compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs:1:5
1 | use rustc_middle::ty;
  |     ^^^^^^^^^^^^^^^^
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: variable does not need to be mutable
   --> compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:327:29
    |
    |
327 |                         let mut trait_predicate = self.resolve_vars_if_possible(trait_predicate);
    |                             |
    |                             help: remove this `mut`
    |
    |
    = note: `-D unused-mut` implied by `-D warnings`
error: variable does not need to be mutable
   --> compiler/rustc_trait_selection/src/traits/specialize/mod.rs:524:10
    |
    |
524 |     for (mut p, _) in predicates {
    |          |
    |          help: remove this `mut`

error: variable does not need to be mutable
