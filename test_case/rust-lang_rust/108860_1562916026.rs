plain
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_transmute v0.1.0 (/checkout/compiler/rustc_transmute)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0004]: non-exhaustive patterns: `AliasKind::Weak` not covered
  --> compiler/rustc_trait_selection/src/solve/project_goals.rs:25:15
   |
25 |         match goal.predicate.projection_ty.kind(self.tcx()) {
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `AliasKind::Weak` not covered
   |
note: `AliasKind` defined here
  --> /checkout/compiler/rustc_type_ir/src/sty.rs:49:5
38 | pub enum AliasKind {
   | ------------------
...
49 |     Weak,
49 |     Weak,
   |     ^^^^ not covered
   = note: the matched value is of type `AliasKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
43 ~             ty::AliasKind::Inherent => bug!("IATs not supported here yet"),
44 ~             AliasKind::Weak => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_trait_selection` (lib test) due to previous error
warning: build failed, waiting for other jobs to finish...
