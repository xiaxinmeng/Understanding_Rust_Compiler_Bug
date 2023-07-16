plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.61 (/checkout/src/tools/clippy/clippy_lints)
error[E0026]: struct `AdtDef` does not have a field named `did`
   --> src/tools/clippy/clippy_lints/src/significant_drop_in_scrutinee.rs:163:30
    |
163 |         if let Some(AdtDef { did, .. }) = ty.ty_adt_def() {
    |                              |
    |                              |
    |                              struct `AdtDef` does not have this field
    |                              help: `AdtDef` has a field named `0`
For more information about this error, try `rustc --explain E0026`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:38
