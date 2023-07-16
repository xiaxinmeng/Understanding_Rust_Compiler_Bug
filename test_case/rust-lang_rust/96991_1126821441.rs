plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0308]: mismatched types
  --> compiler/rustc_mir_transform/src/single_enum.rs:34:33
   |
33 |                 match single_enum_variants.get().get(&src_local) {
   |                       ------------------------------------------ this expression has type `Option<&PackedU8JoinSemiLattice>`
34 |                     None | Some(Fact::TOP) => {}
   |                                 ^^^^^^^^^ expected `&PackedU8JoinSemiLattice`, found struct `PackedU8JoinSemiLattice`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_mir_transform` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_mir_transform` due to previous error
