plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0425]: cannot find value `tcx` in this scope
   --> compiler/rustc_mir_build/src/thir/cx/expr.rs:685:66
    |
685 |                         let const_literal = ty::Const::from_bits(tcx, offset as u128, var_ty);
    |                                                                  ^^^ help: you might have meant to use the available field: `self.tcx`
error[E0308]: mismatched types
   --> compiler/rustc_mir_build/src/thir/cx/expr.rs:685:87
    |
    |
685 |                         let const_literal = ty::Const::from_bits(tcx, offset as u128, var_ty);
    |                                                                                       ^^^^^^ expected struct `ParamEnvAnd`, found struct `rustc_middle::ty::Ty`
    |
    = note: expected struct `ParamEnvAnd<'_, rustc_middle::ty::Ty<'_>, >`
               found struct `rustc_middle::ty::Ty<'_>`
Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_mir_build` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
