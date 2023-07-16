plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0432]: unresolved import `crate::session_diagnostics::RequireStaticErr`
  --> compiler/rustc_borrowck/src/diagnostics/region_errors.rs:30:32
   |
30 |     LifeTimeReturnCategoryErr, RequireStaticErr,
   |                                ^^^^^^^^^^^^^^^^ no `RequireStaticErr` in `session_diagnostics`
error[E0308]: mismatched types
   --> compiler/rustc_borrowck/src/diagnostics/region_errors.rs:685:67
    |
    |
685 |         let mut err = LifeTimeOutLiveErr { span: *span, category: None };
    |                                                                   ^^^^ expected enum `LifeTimeReturnCategoryErr`, found enum `Option`
    |
    = note: expected enum `LifeTimeReturnCategoryErr`

error[E0308]: mismatched types
   --> compiler/rustc_borrowck/src/diagnostics/region_errors.rs:705:24
    |
    |
705 |         err.category = Some(err_category);
    |         ------------   ^^^^^^^^^^^^^^^^^^ expected enum `LifeTimeReturnCategoryErr`, found enum `Option`
    |         expected due to the type of this binding
    |
    |
    = note: expected enum `LifeTimeReturnCategoryErr`
               found enum `Option<LifeTimeReturnCategoryErr>`
Some errors have detailed explanations: E0308, E0432.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_borrowck` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
