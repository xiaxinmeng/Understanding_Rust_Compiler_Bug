plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0261]: use of undeclared lifetime name `'tcx`
   |
   |
72 | impl TraitObligation<'tcx> {
   |     -                ^^^^ undeclared lifetime
   |     |
   |     help: consider introducing lifetime `'tcx` here: `<'tcx>`
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
For more information about this error, try `rustc --explain E0261`.
error: could not compile `rustc_infer` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
