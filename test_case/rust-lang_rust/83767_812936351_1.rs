
error[E0308]: mismatched types
   --> compiler/rustc_symbol_mangling/src/v0.rs:511:55
    |
511 |                         traits::supertraits(self.tcx, projection.trait_ref(self.tcx)).find(|r| {
    |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Binder`, found struct `ExistentialTraitRef`
    |
    = note: expected struct `Binder<'_, rustc_middle::ty::TraitRef<'_>>`
               found struct `ExistentialTraitRef<'_>`

error[E0308]: mismatched types
   --> /Users/user/rust4/library/core/src/macros/mod.rs:39:35
    |
35  | / macro_rules! assert_eq {
36  | |     ($left:expr, $right:expr $(,)?) => ({
37  | |         match (&$left, &$right) {
38  | |             (left_val, right_val) => {
39  | |                 if !(*left_val == *right_val) {
    | |                                   ^^^^^^^^^^ expected struct `ExistentialTraitRef`, found enum `Option`
...   |
61  | |     });
62  | | }
    | |_- in this expansion of `assert_eq!`
    | 
   ::: compiler/rustc_symbol_mangling/src/v0.rs:522:21
    |
522 |                       assert_eq!(last_trait_ref.skip_binder(), assoc_item_parent_trait);
    |                       ------------------------------------------------------------------ in this macro invocation
    |
    = note: expected struct `ExistentialTraitRef<'_>`
                 found enum `Option<Binder<'_, rustc_middle::ty::TraitRef<'_>>>`

error: aborting due to 2 previous errors
