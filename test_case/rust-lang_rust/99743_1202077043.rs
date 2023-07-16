plain
    Checking clippy_lints v0.1.64 (/checkout/src/tools/clippy/clippy_lints)
error: unused import: `TraitEngine`
  --> src/tools/clippy/clippy_lints/src/future_not_send.rs:12:61
   |
12 | use rustc_trait_selection::traits::{self, FulfillmentError, TraitEngine};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error[E0308]: arguments to this function are incorrect
   --> src/tools/clippy/clippy_lints/src/future_not_send.rs:83:21
    |
    |
83  |                     traits::fully_solve_bound(&infcx, cx.param_env, cause, ret_ty, send_trait)
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^         ------------  ----- expected `ParamEnv<'_>`, found `ObligationCause<'_>`
    |                                                       |
    |                                                       expected `ObligationCause<'_>`, found `ParamEnv<'tcx>`
note: function defined here
   --> /checkout/compiler/rustc_trait_selection/src/traits/mod.rs:433:8
    |
    |
433 | pub fn fully_solve_bound<'a, 'tcx>(
help: swap these arguments
    |
    |
83  |                     traits::fully_solve_bound(&infcx, cause, cx.param_env, ret_ty, send_trait)

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_lints` due to 2 previous errors
Build completed unsuccessfully in 0:03:17
