plain
   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0277]: `()` is not an iterator
   --> compiler/rustc_mir_transform/src/coverage/graph.rs:420:17
    |
420 |     ) -> Option<impl Iterator<Item = (BasicCoverageBlock, CoverageKind)>> {
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` is not an iterator
    |
    = help: the trait `Iterator` is not implemented for `()`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
