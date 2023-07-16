plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0616]: field `final_ty` of struct `CoerceMany` is private
   --> compiler/rustc_typeck/src/check/_match.rs:121:30
    |
121 |             debug!(?coercion.final_ty);

For more information about this error, try `rustc --explain E0616`.
error: could not compile `rustc_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
