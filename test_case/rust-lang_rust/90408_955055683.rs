plain
    Checking rustc_lint_defs v0.0.0 (/checkout/compiler/rustc_lint_defs)
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking chalk-solve v0.55.0
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
error[E0271]: type mismatch resolving `<[closure@compiler/rustc_hir/src/definitions.rs:442:42: 442:52] as FnOnce<((&LocalDefId, &Option<hir_id::HirId>),)>>::Output == LocalDefId`
   --> compiler/rustc_hir/src/definitions.rs:441:40
    |
441 |     pub fn iter_local_def_id(&self) -> impl Iterator<Item = LocalDefId> + '_ {
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `LocalDefId`, found `&LocalDefId`
    |
    = note: required because of the requirements on the impl of `Iterator` for `std::iter::Map<indexmap::map::Iter<'_, LocalDefId, Option<hir_id::HirId>>, [closure@compiler/rustc_hir/src/definitions.rs:442:42: 442:52]>`

error[E0599]: no method named `iter` found for tuple `(&LocalDefId, &indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>)` in the current scope
   --> compiler/rustc_hir/src/hir_id.rs:114:42
    |
114 |         self.map.iter().flat_map(|la| la.iter())
    |                                          ^^^^ method not found in `(&LocalDefId, &indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>)`

error[E0599]: no method named `iter_enumerated` found for struct `indexmap::map::IndexMap` in the current scope
   --> compiler/rustc_hir/src/hir_id.rs:118:18
    |
118 |         self.map.iter_enumerated().flat_map(|(owner, la)| {
    |                  ^^^^^^^^^^^^^^^ method not found in `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`

error[E0277]: the type `indexmap::map::IndexMap<LocalDefId, Option<hir_id::HirId>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
   --> compiler/rustc_hir/src/definitions.rs:324:9
    |
324 |         self.def_id_to_hir_id[id].unwrap()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ `indexmap::map::IndexMap<LocalDefId, Option<hir_id::HirId>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
    |
    = help: the trait `std::ops::Index<LocalDefId>` is not implemented for `indexmap::map::IndexMap<LocalDefId, Option<hir_id::HirId>, BuildHasherDefault<FxHasher>>`
error[E0308]: mismatched types
   --> compiler/rustc_hir/src/definitions.rs:361:13
    |
361 |             def_id_to_span,
361 |             def_id_to_span,
    |             ^^^^^^^^^^^^^^ expected struct `indexmap::map::IndexMap`, found struct `IndexVec`
    |
    = note: expected struct `indexmap::map::IndexMap<LocalDefId, rustc_span::Span, BuildHasherDefault<FxHasher>>`
               found struct `IndexVec<_, rustc_span::Span>`

error[E0599]: no method named `push` found for struct `indexmap::map::IndexMap` in the current scope
   --> compiler/rustc_hir/src/definitions.rs:405:39
    |
405 |         let _id = self.def_id_to_span.push(span);
    |                                       ^^^^ method not found in `indexmap::map::IndexMap<LocalDefId, rustc_span::Span, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `iter_enumerated` found for struct `indexmap::map::IndexMap` in the current scope
   --> compiler/rustc_hir/src/definitions.rs:424:14
424 |             .iter_enumerated()
424 |             .iter_enumerated()
    |              ^^^^^^^^^^^^^^^ method not found in `indexmap::map::IndexMap<LocalDefId, Option<hir_id::HirId>, BuildHasherDefault<FxHasher>>`

error[E0277]: the type `indexmap::map::IndexMap<LocalDefId, rustc_span::Span, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
   --> compiler/rustc_hir/src/definitions.rs:438:9
    |
438 |         self.def_id_to_span[def_id]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `indexmap::map::IndexMap<LocalDefId, rustc_span::Span, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
    |
    = help: the trait `std::ops::Index<LocalDefId>` is not implemented for `indexmap::map::IndexMap<LocalDefId, rustc_span::Span, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `ensure_contains_elem` found for struct `indexmap::map::IndexMap` in the current scope
  --> compiler/rustc_hir/src/hir_id.rs:79:18
   |
79 |         self.map.ensure_contains_elem(id, IndexVec::new);
   |                  ^^^^^^^^^^^^^^^^^^^^ method not found in `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`

error[E0277]: the type `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
  --> compiler/rustc_hir/src/hir_id.rs:86:27
   |
86 |         let submap = &mut self.map[id.owner];
   |                           ^^^^^^^^^^^^^^^^^^ `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
   |
   = help: the trait `std::ops::Index<LocalDefId>` is not implemented for `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`

error[E0599]: no method named `ensure_contains_elem` found for struct `indexmap::map::IndexMap` in the current scope
  --> compiler/rustc_hir/src/hir_id.rs:95:18
   |
95 |         self.map.ensure_contains_elem(id.owner, IndexVec::new);
   |                  ^^^^^^^^^^^^^^^^^^^^ method not found in `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`

error[E0277]: the type `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
  --> compiler/rustc_hir/src/hir_id.rs:96:27
   |
96 |         let submap = &mut self.map[id.owner];
   |                           ^^^^^^^^^^^^^^^^^^ `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
   |
   = help: the trait `std::ops::Index<LocalDefId>` is not implemented for `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`
error[E0308]: mismatched types
   --> compiler/rustc_hir/src/hir_id.rs:106:22
    |
    |
106 |         self.map.get(id.owner)?.get(id.local_id)
    |                      |
    |                      expected reference, found struct `LocalDefId`
    |                      expected reference, found struct `LocalDefId`
    |                      help: consider borrowing here: `&id.owner`
    = note: expected reference `&_`
                  found struct `LocalDefId`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> compiler/rustc_hir/src/hir_id.rs:106:37
    |
106 |         self.map.get(id.owner)?.get(id.local_id)
    |                                     |
    |                                     |
    |                                     expected reference, found struct `ItemLocalId`
    |                                     help: consider borrowing here: `&id.local_id`
    = note: expected reference `&_`
    = note: expected reference `&_`
                  found struct `ItemLocalId`

error[E0277]: the type `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
   --> compiler/rustc_hir/src/hir_id.rs:110:10
    |
110 |         &self.map[id]
    |          ^^^^^^^^^^^^ `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
    |
    = help: the trait `std::ops::Index<LocalDefId>` is not implemented for `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`

error[E0277]: the type `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
   --> compiler/rustc_hir/src/hir_id.rs:128:10
    |
128 |         &self.map[id.owner][id.local_id]
    |          ^^^^^^^^^^^^^^^^^^ `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
    |
    = help: the trait `std::ops::Index<LocalDefId>` is not implemented for `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`

error[E0277]: the type `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
   --> compiler/rustc_hir/src/hir_id.rs:134:14
    |
134 |         &mut self.map[id.owner][id.local_id]
    |              ^^^^^^^^^^^^^^^^^^ `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` cannot be indexed by `LocalDefId`
    |
    = help: the trait `std::ops::Index<LocalDefId>` is not implemented for `indexmap::map::IndexMap<LocalDefId, indexmap::map::IndexMap<ItemLocalId, T, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`

error[E0277]: can't compare `LocalDefId` with `LocalDefId`
    --> compiler/rustc_hir/src/hir.rs:1974:5
     |
1972 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encodable, Debug)]
     |                                        ---------- in this derive macro expansion
1973 |   pub struct TraitItemId {
1974 |       pub def_id: LocalDefId,
     |       ^^^^^^^^^^^^^^^^^^^^^^ no implementation for `LocalDefId < LocalDefId` and `LocalDefId > LocalDefId`
    ::: /checkout/library/core/src/cmp.rs:1103:1
     |
     |
1103 | / pub macro PartialOrd($item:item) {
1105 | | }
1105 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `PartialOrd` is not implemented for `LocalDefId`
note: required by `std::cmp::PartialOrd::partial_cmp`
     |
     |
1018 |     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;


error[E0277]: the trait bound `LocalDefId: Ord` is not satisfied
    --> compiler/rustc_hir/src/hir.rs:1974:5
     |
1972 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encodable, Debug)]
     |                                                    --- in this derive macro expansion
1973 |   pub struct TraitItemId {
1974 |       pub def_id: LocalDefId,
     |       ^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `LocalDefId`
    ::: /checkout/library/core/src/cmp.rs:831:1
     |
     |
831  | / pub macro Ord($item:item) {
833  | | }
833  | | }
     | |_- in this expansion of `#[derive(Ord)]`
note: required by `std::cmp::Ord::cmp`
    --> /checkout/library/core/src/cmp.rs:752:5
     |
     |
752  |     fn cmp(&self, other: &Self) -> Ordering;


error[E0277]: can't compare `LocalDefId` with `LocalDefId`
    --> compiler/rustc_hir/src/hir.rs:2037:5
     |
2035 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encodable, Debug)]
     |                                        ---------- in this derive macro expansion
2036 |   pub struct ImplItemId {
2037 |       pub def_id: LocalDefId,
     |       ^^^^^^^^^^^^^^^^^^^^^^ no implementation for `LocalDefId < LocalDefId` and `LocalDefId > LocalDefId`
    ::: /checkout/library/core/src/cmp.rs:1103:1
     |
     |
1103 | / pub macro PartialOrd($item:item) {
1105 | | }
1105 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `PartialOrd` is not implemented for `LocalDefId`
note: required by `std::cmp::PartialOrd::partial_cmp`
     |
     |
1018 |     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;


error[E0277]: the trait bound `LocalDefId: Ord` is not satisfied
    --> compiler/rustc_hir/src/hir.rs:2037:5
     |
2035 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encodable, Debug)]
     |                                                    --- in this derive macro expansion
2036 |   pub struct ImplItemId {
2037 |       pub def_id: LocalDefId,
     |       ^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `LocalDefId`
    ::: /checkout/library/core/src/cmp.rs:831:1
     |
     |
831  | / pub macro Ord($item:item) {
833  | | }
833  | | }
     | |_- in this expansion of `#[derive(Ord)]`
note: required by `std::cmp::Ord::cmp`
    --> /checkout/library/core/src/cmp.rs:752:5
     |
     |
752  |     fn cmp(&self, other: &Self) -> Ordering;


error[E0277]: can't compare `LocalDefId` with `LocalDefId`
    --> compiler/rustc_hir/src/hir.rs:2639:5
     |
2637 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encodable, Debug, Hash)]
     |                                        ---------- in this derive macro expansion
2638 |   pub struct ItemId {
2639 |       pub def_id: LocalDefId,
     |       ^^^^^^^^^^^^^^^^^^^^^^ no implementation for `LocalDefId < LocalDefId` and `LocalDefId > LocalDefId`
    ::: /checkout/library/core/src/cmp.rs:1103:1
     |
     |
1103 | / pub macro PartialOrd($item:item) {
1105 | | }
1105 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `PartialOrd` is not implemented for `LocalDefId`
note: required by `std::cmp::PartialOrd::partial_cmp`
     |
     |
1018 |     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;


error[E0277]: the trait bound `LocalDefId: Ord` is not satisfied
    --> compiler/rustc_hir/src/hir.rs:2639:5
     |
2637 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encodable, Debug, Hash)]
     |                                                    --- in this derive macro expansion
2638 |   pub struct ItemId {
2639 |       pub def_id: LocalDefId,
     |       ^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `LocalDefId`
    ::: /checkout/library/core/src/cmp.rs:831:1
     |
     |
831  | / pub macro Ord($item:item) {
833  | | }
833  | | }
     | |_- in this expansion of `#[derive(Ord)]`
note: required by `std::cmp::Ord::cmp`
    --> /checkout/library/core/src/cmp.rs:752:5
     |
     |
752  |     fn cmp(&self, other: &Self) -> Ordering;


error[E0277]: can't compare `LocalDefId` with `LocalDefId`
    --> compiler/rustc_hir/src/hir.rs:2876:5
     |
2874 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encodable, Debug)]
     |                                        ---------- in this derive macro expansion
2875 |   pub struct ForeignItemId {
2876 |       pub def_id: LocalDefId,
     |       ^^^^^^^^^^^^^^^^^^^^^^ no implementation for `LocalDefId < LocalDefId` and `LocalDefId > LocalDefId`
    ::: /checkout/library/core/src/cmp.rs:1103:1
     |
     |
1103 | / pub macro PartialOrd($item:item) {
1105 | | }
1105 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `PartialOrd` is not implemented for `LocalDefId`
note: required by `std::cmp::PartialOrd::partial_cmp`
     |
     |
1018 |     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;


error[E0277]: the trait bound `LocalDefId: Ord` is not satisfied
    --> compiler/rustc_hir/src/hir.rs:2876:5
     |
2874 |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Encodable, Debug)]
     |                                                    --- in this derive macro expansion
2875 |   pub struct ForeignItemId {
2876 |       pub def_id: LocalDefId,
     |       ^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `LocalDefId`
    ::: /checkout/library/core/src/cmp.rs:831:1
     |
     |
831  | / pub macro Ord($item:item) {
833  | | }
833  | | }
     | |_- in this expansion of `#[derive(Ord)]`
note: required by `std::cmp::Ord::cmp`
    --> /checkout/library/core/src/cmp.rs:752:5
     |
     |
752  |     fn cmp(&self, other: &Self) -> Ordering;


error[E0277]: can't compare `LocalDefId` with `LocalDefId`
    --> compiler/rustc_hir/src/hir_id.rs:19:5
     |
16   |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
     |                                                     ---------- in this derive macro expansion
...
19   |       pub owner: LocalDefId,
     |       ^^^^^^^^^^^^^^^^^^^^^ no implementation for `LocalDefId < LocalDefId` and `LocalDefId > LocalDefId`
    ::: /checkout/library/core/src/cmp.rs:1103:1
     |
     |
1103 | / pub macro PartialOrd($item:item) {
1105 | | }
1105 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `PartialOrd` is not implemented for `LocalDefId`
note: required by `std::cmp::PartialOrd::partial_cmp`
     |
     |
1018 |     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;


error[E0277]: the trait bound `LocalDefId: Ord` is not satisfied
   --> compiler/rustc_hir/src/hir_id.rs:19:5
    |
16  |   #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
    |                                                                 --- in this derive macro expansion
...
19  |       pub owner: LocalDefId,
    |       ^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `LocalDefId`
   ::: /checkout/library/core/src/cmp.rs:831:1
    |
    |
831 | / pub macro Ord($item:item) {
833 | | }
833 | | }
    | |_- in this expansion of `#[derive(Ord)]`
note: required by `std::cmp::Ord::cmp`
   --> /checkout/library/core/src/cmp.rs:752:5
    |
    |
752 |     fn cmp(&self, other: &Self) -> Ordering;

Some errors have detailed explanations: E0271, E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0271`.
error: could not compile `rustc_hir` due to 27 previous errors
