plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0560]: struct `ExternEntry` has no field named `force`
   |
70 |         force: false,
70 |         force: false,
   |         ^^^^^ `ExternEntry` does not have this field
   |
   = note: available fields are: `location`, `is_private_dep`, `add_prelude`, `nounused_dep`, `force_extern`
For more information about this error, try `rustc --explain E0560`.
error: could not compile `rustc_interface` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:14
