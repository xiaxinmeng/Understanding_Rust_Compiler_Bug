plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0308]: mismatched types
   --> compiler/rustc_trait_selection/src/traits/mod.rs:781:70
    |
781 |         ty::Binder::dummy(ty::TraitPredicate { trait_ref, constness: hir::Constness::NotConst }),
    |                                                                      ^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `BoundConstness`, found enum `Constness`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
