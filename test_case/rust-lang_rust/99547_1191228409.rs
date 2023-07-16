plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0609]: no field `infcx` on type `&mut Equate<'_, '_, 'tcx>`
  --> compiler/rustc_infer/src/infer/equate.rs:84:22
   |
84 |                 self.infcx.super_combine_tys(self, a, b)
   |
   |
   = note: available fields are: `fields`, `a_is_expected`
   |
   |
84 |                 self.fields.infcx.super_combine_tys(self, a, b)

    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_infer` due to previous error
