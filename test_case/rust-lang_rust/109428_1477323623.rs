plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
error[E0432]: unresolved import `rustc_middle::ty::DefIdTree`
 --> compiler/rustc_lint/src/opaque_hidden_inferred_bound.rs:5:72
  |
5 |     self, fold::BottomUpFolder, print::TraitPredPrintModifiersAndPath, DefIdTree, Ty, TypeFoldable,
  |                                                                        ^^^^^^^^^ no `DefIdTree` in `ty`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustc_lint` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_lint` due to previous error
