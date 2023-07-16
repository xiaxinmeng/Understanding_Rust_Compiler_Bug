plain
   |
20 | use std::cmp::Ordering;
   |     ^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `adt::AdtDefData: Ord` is not satisfied
   --> compiler/rustc_middle/src/ty/adt.rs:151:25
    |
149 |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, HashStable)]
150 |   #[rustc_pass_by_value]
150 |   #[rustc_pass_by_value]
151 |   pub struct AdtDef<'tcx>(pub Interned<'tcx, AdtDefData>);
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `adt::AdtDefData`
   ::: /checkout/library/core/src/cmp.rs:860:1
    |
    |
860 | / pub macro Ord($item:item) {
862 | | }
862 | | }
    | |_- in this expansion of `#[derive(Ord)]`
    |
    = help: the trait `Ord` is implemented for `Interned<'a, T>`
    = note: required because of the requirements on the impl of `Ord` for `Interned<'_, adt::AdtDefData>`

error[E0277]: can't compare `adt::AdtDefData` with `adt::AdtDefData`
    --> compiler/rustc_middle/src/ty/adt.rs:151:25
     |
149  |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, HashStable)]
150  |   #[rustc_pass_by_value]
150  |   #[rustc_pass_by_value]
151  |   pub struct AdtDef<'tcx>(pub Interned<'tcx, AdtDefData>);
     |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `adt::AdtDefData < adt::AdtDefData` and `adt::AdtDefData > adt::AdtDefData`
    ::: /checkout/library/core/src/cmp.rs:1169:1
     |
1169 | / pub macro PartialOrd($item:item) {
1170 | |     /* compiler built-in */
1170 | |     /* compiler built-in */
1171 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `adt::AdtDefData`
     = note: required because of the requirements on the impl of `std::cmp::PartialOrd<Interned<'_, adt::AdtDefData>>` for `Interned<'_, adt::AdtDefData>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to 3 previous errors
