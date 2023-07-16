plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0425]: cannot find value `base` in this scope
    --> compiler/rustc_typeck/src/check/expr.rs:2213:44
     |
2213 |                     self.tcx.parent_module(base.hir_id).to_def_id(),

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
