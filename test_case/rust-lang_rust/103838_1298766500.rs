plain
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
error[E0061]: this function takes 4 arguments but 0 arguments were supplied
    --> compiler/rustc_hir_typeck/src/expr.rs:1852:55
     |
1852 |         if let Some(last) = ast_fields.last() && self.suggest_fru_from_range() {
     |                                                       ^^^^^^^^^^^^^^^^^^^^^^-- multiple arguments are missing
note: associated function defined here
    --> compiler/rustc_hir_typeck/src/expr.rs:1864:8
     |
     |
1864 |     fn suggest_fru_from_range(
1865 |         &self,
1865 |         &self,
1866 |         last_expr_field: &hir::ExprField<'tcx>,
1867 |         variant: &ty::VariantDef,
     |         ------------------------
     |         ------------------------
1868 |         substs: SubstsRef<'tcx>,
1869 |         err: &mut Diagnostic,
     |         --------------------
help: provide the arguments
     |
     |
1852 |         if let Some(last) = ast_fields.last() && self.suggest_fru_from_range(/* &rustc_hir::ExprField<'_> */, /* &VariantDef */, /* &rustc_middle::ty::List<rustc_middle::ty::GenericArg<'_>> */, /* &mut Diagnostic */) {

error[E0308]: mismatched types
    --> compiler/rustc_hir_typeck/src/expr.rs:1852:50
     |
     |
1852 |         if let Some(last) = ast_fields.last() && self.suggest_fru_from_range() {
     |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`
error[E0308]: mismatched types
    --> compiler/rustc_hir_typeck/src/expr.rs:1815:10
     |
1807 |     fn report_missing_fields(
1807 |     fn report_missing_fields(
     |        --------------------- implicitly returns `()` as its body has no tail or `return` expression
1815 |     ) -> bool {
     |          ^^^^ expected `bool`, found `()`

Some errors have detailed explanations: E0061, E0308.
