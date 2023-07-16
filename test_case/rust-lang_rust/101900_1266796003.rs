plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.6.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.65 (/checkout/src/tools/clippy/clippy_lints)
error[E0599]: no method named `has_param_types_or_consts` found for reference `&rustc_middle::ty::List<rustc_middle::ty::GenericArg<'_>>` in the current scope
     |
     |
1241 |             ty::Adt(_, substs) if substs.has_param_types_or_consts() => {
     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&rustc_middle::ty::List<rustc_middle::ty::GenericArg<'_>>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:04:18
