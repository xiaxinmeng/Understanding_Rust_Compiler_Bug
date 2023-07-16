plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/clean/auto_trait.rs:151:25
    |
151 |         if !ty.is_sized(tcx.at(rustc_span::DUMMY_SP), param_env) {
    |                -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc_middle::ty::TyCtxt`, found struct `TyCtxtAt`
    |                arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/util.rs:833:12
   --> /checkout/compiler/rustc_middle/src/ty/util.rs:833:12
    |
833 |     pub fn is_sized(self, tcx: TyCtxt<'tcx>, param_env: ty::ParamEnv<'tcx>) -> bool {
help: consider dereferencing the type
    |
    |
151 |         if !ty.is_sized(*tcx.at(rustc_span::DUMMY_SP), param_env) {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:21
