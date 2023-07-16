plain
    Checking clippy_lints v0.1.66 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/future_not_send.rs:81:65
    |
81  |                 let cause = traits::ObligationCause::misc(span, hir_id);
    |                             -----------------------------       ^^^^^^ expected struct `LocalDefId`, found struct `HirId`
    |                             arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/traits/mod.rs:130:12
   --> /checkout/compiler/rustc_middle/src/traits/mod.rs:130:12
    |
130 |     pub fn misc(span: Span, body_id: def_id::LocalDefId) -> ObligationCause<'tcx> {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:04:38
