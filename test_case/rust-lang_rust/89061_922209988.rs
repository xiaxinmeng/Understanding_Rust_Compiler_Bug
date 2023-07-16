plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0425]: cannot find value `tr` in this scope
    --> compiler/rustc_typeck/src/astconv/mod.rs:1309:47
     |
1309 |                 let trait_def = tcx.trait_def(tr.def_id);
     |                                               ^^ not found in this scope
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
