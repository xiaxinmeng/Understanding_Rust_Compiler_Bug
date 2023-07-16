plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0425]: cannot find value `expr` in this scope
    |
    |
462 |                         ty: self.typeck_results().node_type(expr.hir_id),
    |
help: consider importing one of these items
    |
11  | use crate::traits::sym::expr;
11  | use crate::traits::sym::expr;
    |
11  | use rustc_span::sym::expr;
    |

error[E0599]: no method named `typeck_results` found for mutable reference `&mut AbstractConstBuilder<'a, 'tcx>` in the current scope
    |
    |
462 |                         ty: self.typeck_results().node_type(expr.hir_id),
    |                                  ^^^^^^^^^^^^^^ method not found in `&mut AbstractConstBuilder<'a, 'tcx>`
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
Some errors have detailed explanations: E0425, E0599.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustc_trait_selection` due to 2 previous errors
