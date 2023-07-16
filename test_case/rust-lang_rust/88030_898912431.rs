plain
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
error: unused import: `PredicateKind`
  --> compiler/rustc_traits/src/type_op.rs:10:35
   |
10 |     self, FnSig, Lift, PolyFnSig, PredicateKind, Ty, TyCtxt, TypeFoldable, Variance,
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_traits` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:22
