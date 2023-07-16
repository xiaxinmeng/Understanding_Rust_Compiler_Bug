plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0282]: type annotations needed
   --> compiler/rustc_trait_selection/src/traits/coherence.rs:187:30
    |
187 |     let negate_obligation = |mut obligation| {
    |                              ^^^^^^^^^^^^^^ consider giving this closure parameter a type
    = note: type must be known at this point


error[E0599]: no method named `predicate_must_hold` found for mutable reference `&mut traits::select::SelectionContext<'cx, 'tcx>` in the current scope
    |
    |
217 |                 if selcx.predicate_must_hold(o_neg) {
    |                          ^^^^^^^^^^^^^^^^^^^ method not found in `&mut traits::select::SelectionContext<'cx, 'tcx>`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0599.
For more information about an error, try `rustc --explain E0282`.
