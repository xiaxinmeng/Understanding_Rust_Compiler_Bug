plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error[E0026]: struct `TraitPredicate` does not have a field named `constness`
   --> compiler/rustc_privacy/src/lib.rs:130:17
130 |                 constness: _,
130 |                 constness: _,
    |                 ^^^^^^^^^ struct `TraitPredicate` does not have this field
error[E0308]: mismatched types
    --> compiler/rustc_privacy/src/lib.rs:1153:17
     |
     |
1153 |             for (trait_predicate, _, _) in bounds.trait_bounds {
     |                 ^^^^^^^^^^^^^^^^^^^^^^^    ------------------- this expression has type `Option<(Binder<'_, rustc_middle::ty::TraitRef<'_>>, rustc_span::Span)>`
     |                 |
     |                 expected a tuple with 2 elements, found one with 3 elements
     |
     = note: expected tuple `(Binder<'_, rustc_middle::ty::TraitRef<'_>>, rustc_span::Span)`
                found tuple `(_, _, _)`
Some errors have detailed explanations: E0026, E0308.
For more information about an error, try `rustc --explain E0026`.
error: could not compile `rustc_privacy` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
