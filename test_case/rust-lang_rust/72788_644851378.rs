
error[E0560]: struct `rustc_middle::traits::ObligationCause<'_>` has no field named `code`
    --> src/librustc_typeck/check/compare_method.rs:1241:13
     |
1241 |             code: ObligationCauseCode::ItemObligation(trait_ty.def_id),
     |             ^^^^ `rustc_middle::traits::ObligationCause<'_>` does not have this field
    Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
error: aborting due to 3 previous errors
