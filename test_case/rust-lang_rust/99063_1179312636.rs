plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.6.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.64 (/checkout/src/tools/clippy/clippy_lints)
error[E0609]: no field `lint_reasons` on type `&rustc_feature::active::Features`
    |
    |
454 |     if !cx.tcx.sess.features_untracked().lint_reasons {
    |
    |
    = note: available fields are: `declared_lang_features`, `declared_lib_features`, `active_features`, `abi_thiscall`, `abi_unadjusted` ... and 177 others
For more information about this error, try `rustc --explain E0609`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:44
