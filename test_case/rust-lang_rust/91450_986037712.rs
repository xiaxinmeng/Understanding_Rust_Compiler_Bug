plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0599]: no method named `without_const` found for struct `Binder` in the current scope
     |
     |
608  | ...                   Some((identity_trait_ref.without_const().to_predicate(tcx), item.span));
     |                                                ^^^^^^^^^^^^^ method not found in `Binder<'_, rustc_middle::ty::TraitRef<'_>>`
    ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:1392:8
     |
     |
1392 |     fn without_const(self) -> ConstnessAnd<Self> {
     |        ------------- the method is available for `Binder<'_, rustc_middle::ty::TraitRef<'_>>` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
     |
17   | use crate::rustc_middle::ty::WithConstness;


error[E0599]: no method named `without_const` found for struct `Binder` in the current scope
     |
     |
1999 |                 ty::TraitRef::identity(tcx, def_id).without_const().to_predicate(tcx),
     |                                                     ^^^^^^^^^^^^^ method not found in `Binder<'_, rustc_middle::ty::TraitRef<'_>>`
    ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:1392:8
     |
     |
1392 |     fn without_const(self) -> ConstnessAnd<Self> {
     |        ------------- the method is available for `Binder<'_, rustc_middle::ty::TraitRef<'_>>` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
     |
17   | use crate::rustc_middle::ty::WithConstness;


error[E0599]: no method named `without_const` found for struct `Binder` in the current scope
     |
     |
2117 |         predicates.insert((trait_ref.without_const().to_predicate(tcx), tcx.def_span(def_id)));
     |                                      ^^^^^^^^^^^^^ method not found in `Binder<'_, rustc_middle::ty::TraitRef<'_>>`
    ::: /checkout/compiler/rustc_middle/src/ty/mod.rs:1392:8
     |
     |
1392 |     fn without_const(self) -> ConstnessAnd<Self> {
     |        ------------- the method is available for `Binder<'_, rustc_middle::ty::TraitRef<'_>>` here
     = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
     |
     |
17   | use crate::rustc_middle::ty::WithConstness;

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_typeck` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
