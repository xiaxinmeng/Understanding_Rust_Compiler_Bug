plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0425]: cannot find value `error_def_id` in this scope
  --> compiler/rustc_typeck/src/coherence/inherent_impls.rs:59:70
   |
59 |             ty::Dynamic(data, ..) if data.principal_def_id() == Some(error_def_id) => {

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
