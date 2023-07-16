plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_lint/src/enum_intrinsics_non_enums.rs:41:24
    |
41  |     !t.is_enum() && !t.definitely_needs_subst()
    |                        |
    |                        expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/fold.rs:132:8
    |
132 |     fn definitely_needs_subst(&self, tcx: TyCtxt<'tcx>) -> bool {

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustc_lint` due to previous error
warning: build failed, waiting for other jobs to finish...
