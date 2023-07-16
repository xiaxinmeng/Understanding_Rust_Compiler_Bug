plain
    Checking rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
    Checking rustc_arena v0.0.0 (/checkout/compiler/rustc_arena)
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error[E0277]: the trait bound `LocalDefId: std::cmp::Ord` is not satisfied
    |
    |
433 |   #[derive(Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
    |                                              --- in this derive macro expansion
...
440 |       pub parent: Option<LocalDefId>,
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::cmp::Ord` is not implemented for `LocalDefId`
   ::: /checkout/library/core/src/cmp.rs:831:1
    |
    |
831 | / pub macro Ord($item:item) {
833 | | }
833 | | }
    | |_- in this expansion of `#[derive(Ord)]`
    |
    = note: required because of the requirements on the impl of `std::cmp::Ord` for `Option<LocalDefId>`
note: required by `std::cmp::Ord::cmp`
    |
    |
752 |     fn cmp(&self, other: &Self) -> Ordering;


error[E0277]: can't compare `LocalDefId` with `LocalDefId`
     |
     |
433  |   #[derive(Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
     |                                                   ---------- in this derive macro expansion
...
440  |       pub parent: Option<LocalDefId>,
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `LocalDefId < LocalDefId` and `LocalDefId > LocalDefId`
    ::: /checkout/library/core/src/cmp.rs:1103:1
     |
     |
1103 | / pub macro PartialOrd($item:item) {
1105 | | }
1105 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `PartialOrd` is not implemented for `LocalDefId`
     = note: required because of the requirements on the impl of `PartialOrd` for `Option<LocalDefId>`
note: required by `std::cmp::PartialOrd::partial_cmp`
     |
     |
1018 |     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_span` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
