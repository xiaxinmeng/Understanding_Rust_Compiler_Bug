plain
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0407]: method `tcx_for_anon_const_substs` is not a member of trait `TypeVisitor`
    --> compiler/rustc_middle/src/ty/sty.rs:1991:13
     |
1991 | /             fn tcx_for_anon_const_substs(&self) -> Option<TyCtxt<'tcx>> {
1992 | |                 Some(self.1)
     | |_____________^ not a member of trait `TypeVisitor`

   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
For more information about this error, try `rustc --explain E0407`.
