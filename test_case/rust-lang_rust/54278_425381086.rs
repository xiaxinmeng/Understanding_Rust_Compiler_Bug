plain
[00:14:20]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:14:32] error[E0308]: mismatched types
[00:14:32]    --> librustc/infer/outlives/verify.rs:305:47
[00:14:32]     |
[00:14:32] 305 |             traits::elaborate_predicates(tcx, trait_predicates.predicates),
[00:14:32]     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `ty::Predicate`, found tuple
[00:14:32]     |
[00:14:32]     = note: expected type `std::vec::Vec<ty::Predicate<'_>>`
[00:14:32]                found type `std::vec::Vec<(ty::Predicate<'_>, syntax_pos::Span)>`
[00:14:32] error: aborting due to previous error
[00:14:32] 
[00:14:32] For more information about this error, try `rustc --explain E0308`.
[00:14:32] error: Could not compile `rustc`.
