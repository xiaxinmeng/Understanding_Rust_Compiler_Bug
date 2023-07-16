
error[E0004]: non-exhaustive patterns: `Err(NormalizationFailure(_, _))` not covered
681
    --> src/librustdoc/html/render/print_item.rs:1705:11
682
     |
683
1705 |     match tcx.layout_of(param_env.and(ty)) {
684
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `Err(NormalizationFailure(_, _))` not covered
685
     |
686
    ::: /checkout/library/core/src/result.rs:512:5
687
     |
688
512  |     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
689
     |     --- not covered
690
     |
691
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
692
     = note: the matched value is of type `Result<TyAndLayout<&TyS>, rustc_middle::ty::layout::LayoutError>`
693

