plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: unused variable: `minimum_input_count`
   --> compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:202:13
    |
202 |         let minimum_input_count = expected_input_tys.len();
    |             ^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_minimum_input_count`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: variable does not need to be mutable
error: variable does not need to be mutable
   --> compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:309:49
    |
309 |         if let Some((expected_count, arg_count, mut err_code, sugg_unit)) = error {
    |                                                 |
    |                                                 help: remove this `mut`
    |
    |
    = note: `-D unused-mut` implied by `-D warnings`
error: could not compile `rustc_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:02:50
