
error[E0308]: mismatched types
   --> compiler/rustc_symbol_mangling/src/v0.rs:513:55
    |
513 |                         traits::supertraits(self.tcx, projection.trait_ref(self.tcx).with_self_ty(self.tcx, dummy_self)).find(|r| {
    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Binder`, found struct `rustc_middle::ty::TraitRef`
    |
    = note: expected struct `Binder<'_, rustc_middle::ty::TraitRef<'_>, >`
               found struct `rustc_middle::ty::TraitRef<'_>`
