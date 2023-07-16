plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0308]: mismatched types
   --> src/librustdoc/clean/mod.rs:142:60
    |
142 |             let trait_ = clean_trait_ref_with_bindings(cx, trait_ref, bindings);
    |                          -----------------------------     ^^^^^^^^^ expected `Binder<'_, TraitRef<'_>>`, found `TraitRef<'_>`
    |                          arguments to this function are incorrect
    |
    |
    = note: expected struct `rustc_middle::ty::Binder<'_, rustc_middle::ty::TraitRef<'_>, >`
               found struct `rustc_middle::ty::TraitRef<'_>`
   --> src/librustdoc/clean/mod.rs:161:15
    |
    |
161 | pub(crate) fn clean_trait_ref_with_bindings<'tcx>(
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
162 |     cx: &mut DocContext<'tcx>,
163 |     trait_ref: ty::PolyTraitRef<'tcx>,

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:01:13
