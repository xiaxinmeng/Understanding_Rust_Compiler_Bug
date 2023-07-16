plain
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error: unused import: `rustc_data_structures::fx::FxHashSet`
  |
  |
4 | use rustc_data_structures::fx::FxHashSet;
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_span::def_id::DefId`
  --> compiler/rustc_lint/src/panic_in_drop.rs:10:5
   |
10 | use rustc_span::def_id::DefId;
