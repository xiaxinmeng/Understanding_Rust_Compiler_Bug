plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no method named `map_or` found for struct `rustc_span::def_id::LocalDefId` in the current scope
   --> compiler/rustc_middle/src/hir/mod.rs:107:30
    |
107 |         tcx.local_parent(id).map_or(CRATE_HIR_ID, |parent| {
    |                              ^^^^^^ method not found in `rustc_span::def_id::LocalDefId`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to previous error
