plain
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
error[E0599]: no variant or associated item named `of_impl` found for enum `std::option::Option` in the current scope
   --> compiler/rustc_privacy/src/lib.rs:783:71
    |
783 |                 if let Some(item_ev) = Option::<EffectiveVisibility>::of_impl(
    |                                                                       ^^^^^^^ variant or associated item not found in `Option<EffectiveVisibility>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_privacy` (lib test) due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_privacy` (lib) due to previous error
