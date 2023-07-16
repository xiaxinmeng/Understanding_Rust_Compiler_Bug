plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `to_predicate` found for struct `ConstnessAnd` in the current scope
  --> src/librustdoc/clean/blanket_impl.rs:67:67
   |
67 | ...                   .chain(Some(trait_ref.without_const().to_predicate(infcx.tcx)));
   |                                                             ^^^^^^^^^^^^ method not found in `ConstnessAnd<rustc_middle::ty::TraitRef<'_>>`
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:144:41
    |
    |
144 |                     PolyTrait { trait_: (trait_ref, &*bindings).clean(cx), generic_params: vec![] },
    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `types::Type`, found enum `types::GenericBound`

error: unused import: `ToPredicate`
 --> src/librustdoc/clean/blanket_impl.rs:6:24
  |
6 | use rustc_middle::ty::{ToPredicate, WithConstness};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to 3 previous errors
Build completed unsuccessfully in 0:03:07
