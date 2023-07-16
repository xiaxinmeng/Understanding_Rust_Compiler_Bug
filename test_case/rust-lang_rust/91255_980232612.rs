plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0004]: non-exhaustive patterns: `Err(NormalizationFailure(_, _))` not covered
    --> src/librustdoc/html/render/print_item.rs:1705:11
     |
1705 |     match tcx.layout_of(param_env.and(ty)) {
     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `Err(NormalizationFailure(_, _))` not covered
    ::: /checkout/library/core/src/result.rs:512:5
     |
     |
512  |     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `Result<TyAndLayout<&TyS>, rustc_middle::ty::layout::LayoutError>`
For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:03:32
