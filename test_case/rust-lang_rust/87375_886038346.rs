plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0277]: the trait bound `ConstnessAnd<Binder<'_, rustc_middle::ty::TraitRef<'_>>>: Relate<'_>` is not satisfied
     |
     |
2047 |         matcher.relate(previous, current).is_ok()
     |                 ^^^^^^ the trait `Relate<'_>` is not implemented for `ConstnessAnd<Binder<'_, rustc_middle::ty::TraitRef<'_>>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_trait_selection`
