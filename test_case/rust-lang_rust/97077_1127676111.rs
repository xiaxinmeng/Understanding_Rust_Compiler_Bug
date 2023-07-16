plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0599]: no method named `ret_deref` found for reference `&rustc_middle::mir::Place<'tcx>` in the current scope
   |
   |
82 |             if place.ret_deref().is_some() {
   |                      ^^^^^^^^^ help: there is an associated function with a similar name: `has_deref`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_mir_transform` due to previous error
