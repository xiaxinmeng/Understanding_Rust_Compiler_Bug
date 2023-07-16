plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0599]: no method named `create_stable_hashing_context` found for struct `TyCtxt<'tcx>` in the current scope
   --> compiler/rustc_borrowck/src/lib.rs:444:58
    |
444 |     opaque_type_values.sort_deterministically(&infcx.tcx.create_stable_hashing_context());
    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `with_stable_hashing_context`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_borrowck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_borrowck` due to previous error
