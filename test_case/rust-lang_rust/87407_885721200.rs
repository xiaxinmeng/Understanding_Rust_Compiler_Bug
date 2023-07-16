plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_trait_selection/src/traits/fulfill.rs:675:33
    |
675 |         if obligation.predicate.is_global() {
    |                                 |
    |                                 expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/fold.rs:163:8
    |
163 |     fn is_global(&self, tcx: TyCtxt<'tcx>) -> bool {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
