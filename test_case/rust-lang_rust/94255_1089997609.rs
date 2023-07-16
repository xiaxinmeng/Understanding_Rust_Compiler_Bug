plain
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0407]: method `visit_constant` is not a member of trait `visit::Visitor`
   --> compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:380:13
    |
380 | /             fn visit_constant(&mut self, ct: mir::ConstantKind<'tcx>) {
381 | |                 self.is_poly |= ct.has_param_types_or_consts();
    | |_____________^ not a member of trait `visit::Visitor`

    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
For more information about this error, try `rustc --explain E0407`.
