plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0004]: non-exhaustive patterns: `&Cast(PointerFromExposedAddress, _, _)` not covered
     |
2611 |         match self {
2611 |         match self {
     |               ^^^^ pattern `&Cast(PointerFromExposedAddress, _, _)` not covered
     |
note: `mir::Rvalue` defined here
     |
2483 | pub enum Rvalue<'tcx> {
     |          ------
...
...
2541 |     Cast(CastKind, Operand<'tcx>, Ty<'tcx>),
     |     ^^^^ not covered
     = note: the matched value is of type `&mir::Rvalue`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
     |
2630 ~             | Rvalue::ShallowInitBox(_, _) => true,
2631 + 
2632 ~             &Cast(PointerFromExposedAddress, _, _) => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
