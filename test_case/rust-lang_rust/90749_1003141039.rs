plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/hir/map/mod.rs:552:48
    |
552 |         self.tcx.all_local_trait_impls(()).get(&trait_did).map_or(&[], |xs| &xs[..])
    |                                            --- ^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
    |                                            required by a bound introduced by this call
    |
note: required by a bound in `std::collections::BTreeMap::<K, V>::get`
   --> /checkout/library/alloc/src/collections/btree/map.rs:558:24
   --> /checkout/library/alloc/src/collections/btree/map.rs:558:24
    |
558 |         K: Borrow<Q> + Ord,
    |                        ^^^ required by this bound in `std::collections::BTreeMap::<K, V>::get`

error[E0599]: no method named `extend` found for mutable reference `&mut std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>` in the current scope
   --> compiler/rustc_middle/src/ty/print/pretty.rs:866:70
    |
866 |                         traits.entry(fn_once_trait_ref).or_default().extend(
    |                                                                      ^^^^^^ method not found in `&mut std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>`

error[E0599]: no method named `extend` found for mutable reference `&mut std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>` in the current scope
   --> compiler/rustc_middle/src/ty/print/pretty.rs:984:46
    |
984 |         traits.entry(trait_ref).or_default().extend(proj_ty);
    |                                              ^^^^^^ method not found in `&mut std::collections::BTreeMap<rustc_span::def_id::DefId, Binder<'tcx, &'tcx TyS<'tcx>>>`

error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/print/pretty.rs:1089:21
1089 |         auto_traits.sort();
1089 |         auto_traits.sort();
     |                     ^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
     |
     = note: required because of the requirements on the impl of `Ord` for `(std::string::String, rustc_span::def_id::DefId)`
note: required by a bound in `std::slice::<impl [T]>::sort`
     |
     |
274  |         T: Ord,
     |            ^^^ required by this bound in `std::slice::<impl [T]>::sort`

error[E0599]: the method `cmp` exists for struct `rustc_span::def_id::DefId`, but its trait bounds were not satisfied
   --> compiler/rustc_middle/src/ty/adt.rs:112:18
    |
112 |         self.did.cmp(&other.did)
    |                  ^^^ method cannot be called on `rustc_span::def_id::DefId` due to unsatisfied trait bounds
   ::: /checkout/compiler/rustc_span/src/def_id.rs:225:1
    |
225 | pub struct DefId {
225 | pub struct DefId {
    | ---------------- doesn't satisfy `rustc_span::def_id::DefId: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `rustc_span::def_id::DefId: Iterator`
            which is required by `&mut rustc_span::def_id::DefId: Iterator`

error[E0599]: the method `hash_stable` exists for reference `&indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>`, but its trait bounds were not satisfied
    |
    |
51  |   #[derive(Default, TyEncodable, TyDecodable, Debug, HashStable)]
    |                                                      |
    |                                                      |
    |                                                      method cannot be called on `&indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>` due to unsatisfied trait bounds
    |                                                      in this derive macro expansion
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/indexmap-1.7.0/src/map.rs:71:1
    |
    |
71  |   pub struct IndexMap<K, V, S = RandomState> {
    |   ------------------------------------------ doesn't satisfy `_: HashStable<_>`
   ::: compiler/rustc_middle/src/ty/fast_reject.rs:21:1
    |
    |
21  | / pub enum SimplifiedTypeGen<D>
22  | | where
23  | |     D: Copy + Debug + Eq,
24  | | {
48  | |     ParameterSimplifiedType,
49  | | }
49  | | }
    | |_- doesn't satisfy `_: HashStable<_>`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(HashStable)]`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `SimplifiedTypeGen<rustc_span::def_id::DefId>: HashStable<_>`
            which is required by `indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>: HashStable<_>`
            `indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>: HashStable<_>`
            which is required by `&indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>: HashStable<_>`
   --> /checkout/compiler/rustc_data_structures/src/stable_hasher.rs:170:1
    |
    |
170 | / pub trait HashStable<CTX> {
171 | |     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher);
    | |_^


error[E0599]: the method `hash_stable` exists for reference `&indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>`, but its trait bounds were not satisfied
   --> compiler/rustc_middle/src/ty/trait_def.rs:67:26
67  |   #[derive(Default, Debug, HashStable)]
    |                            ^^^^^^^^^^
    |                            |
    |                            |
    |                            method cannot be called on `&indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>` due to unsatisfied trait bounds
    |                            in this derive macro expansion
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/indexmap-1.7.0/src/map.rs:71:1
    |
    |
71  |   pub struct IndexMap<K, V, S = RandomState> {
    |   ------------------------------------------ doesn't satisfy `_: HashStable<_>`
   ::: compiler/rustc_middle/src/ty/fast_reject.rs:21:1
    |
    |
21  | / pub enum SimplifiedTypeGen<D>
22  | | where
23  | |     D: Copy + Debug + Eq,
24  | | {
48  | |     ParameterSimplifiedType,
49  | | }
49  | | }
    | |_- doesn't satisfy `_: HashStable<_>`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(HashStable)]`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `SimplifiedTypeGen<rustc_span::def_id::DefId>: HashStable<_>`
            which is required by `indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>: HashStable<_>`
            `indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>: HashStable<_>`
            which is required by `&indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>: HashStable<_>`
   --> /checkout/compiler/rustc_data_structures/src/stable_hasher.rs:170:1
    |
    |
170 | / pub trait HashStable<CTX> {
171 | |     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher);
    | |_^


error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/consts/kind.rs:25:5
     |
22   |   #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, TyEncodable, TyDecodable, Lift)]
     |                                               ---------- in this derive macro expansion
...
25   |       pub def: ty::WithOptConstParam<DefId>,
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`
note: required because of the requirements on the impl of `std::cmp::PartialOrd` for `WithOptConstParam<rustc_span::def_id::DefId>`
    --> compiler/rustc_middle/src/ty/mod.rs:1136:25
     |
1136 |   #[derive(PartialEq, Eq, PartialOrd, Ord)]
     |                           |
     |                           in this derive macro expansion
     |                           in this derive macro expansion
     |
     |
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
1108 |   pub macro PartialOrd($item:item) {
     | |_|
     | |
1109 | |     /* compiler built-in */
1110 | | }
1110 | | }
     | | -
     | |_|
     | |_in this expansion of `#[derive(PartialOrd)]`
     |   in this expansion of `#[derive(PartialOrd)]`

error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/consts/kind.rs:25:5
     |
22   |   #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, TyEncodable, TyDecodable, Lift)]
     |                                                           --- in this derive macro expansion
...
25   |       pub def: ty::WithOptConstParam<DefId>,
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:832:1
     |
     |
832  | / pub macro Ord($item:item) {
834  | | }
834  | | }
     | |_- in this expansion of `#[derive(Ord)]`
     |
note: required because of the requirements on the impl of `Ord` for `WithOptConstParam<rustc_span::def_id::DefId>`
    --> compiler/rustc_middle/src/ty/mod.rs:1136:37
     |
1136 |   #[derive(PartialEq, Eq, PartialOrd, Ord)]
     |                                       |
     |                                       in this derive macro expansion
     |                                       in this derive macro expansion
     |
     |
    ::: /checkout/library/core/src/cmp.rs:832:1
     |
832  |   pub macro Ord($item:item) {
     | |_|
     | |
833  | |     /* compiler built-in */
834  | | }
834  | | }
     | | -
     | |_|
     | |_in this expansion of `#[derive(Ord)]`
     |   in this expansion of `#[derive(Ord)]`

error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/instance.rs:35:10
     |
26   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
     |                                        ---------- in this derive macro expansion
...
35   |       Item(ty::WithOptConstParam<DefId>),
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`
note: required because of the requirements on the impl of `std::cmp::PartialOrd` for `WithOptConstParam<rustc_span::def_id::DefId>`
    --> compiler/rustc_middle/src/ty/mod.rs:1136:25
     |
1136 |   #[derive(PartialEq, Eq, PartialOrd, Ord)]
     |                           |
     |                           in this derive macro expansion
     |                           in this derive macro expansion
     |
     |
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
1108 |   pub macro PartialOrd($item:item) {
     | |_|
     | |
1109 | |     /* compiler built-in */
1110 | | }
1110 | | }
     | | -
     | |_|
     | |_in this expansion of `#[derive(PartialOrd)]`
     |   in this expansion of `#[derive(PartialOrd)]`

error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/instance.rs:42:15
     |
26   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
     |                                        ---------- in this derive macro expansion
42   |       Intrinsic(DefId),
42   |       Intrinsic(DefId),
     |                 ^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`

error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/instance.rs:49:16
     |
26   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
     |                                        ---------- in this derive macro expansion
...
49   |       VtableShim(DefId),
     |                  ^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`

error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/instance.rs:62:15
     |
26   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
     |                                        ---------- in this derive macro expansion
...
62   |       ReifyShim(DefId),
     |                 ^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`

error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/instance.rs:67:15
     |
26   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
     |                                        ---------- in this derive macro expansion
...
67   |       FnPtrShim(DefId, Ty<'tcx>),
     |                 ^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`

error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/instance.rs:76:13
     |
26   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
     |                                        ---------- in this derive macro expansion
...
76   |       Virtual(DefId, usize),
     |               ^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`

error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/instance.rs:81:23
     |
26   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
     |                                        ---------- in this derive macro expansion
...
81   |       ClosureOnceShim { call_once: DefId, track_caller: bool },
     |                         ^^^^^^^^^^^^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`

error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/instance.rs:88:14
     |
26   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
     |                                        ---------- in this derive macro expansion
...
88   |       DropGlue(DefId, Option<Ty<'tcx>>),
     |                ^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`

error[E0277]: can't compare `rustc_span::def_id::DefId` with `rustc_span::def_id::DefId`
    --> compiler/rustc_middle/src/ty/instance.rs:96:15
     |
26   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
     |                                        ---------- in this derive macro expansion
...
96   |       CloneShim(DefId, Ty<'tcx>),
     |                 ^^^^^ no implementation for `rustc_span::def_id::DefId < rustc_span::def_id::DefId` and `rustc_span::def_id::DefId > rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:1108:1
     |
     |
1108 | / pub macro PartialOrd($item:item) {
1110 | | }
1110 | | }
     | |_- in this expansion of `#[derive(PartialOrd)]`
     |
     = help: the trait `std::cmp::PartialOrd` is not implemented for `rustc_span::def_id::DefId`

error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
    --> compiler/rustc_middle/src/ty/instance.rs:35:10
     |
26   |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
     |                                                    --- in this derive macro expansion
...
35   |       Item(ty::WithOptConstParam<DefId>),
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
    ::: /checkout/library/core/src/cmp.rs:832:1
     |
     |
832  | / pub macro Ord($item:item) {
834  | | }
834  | | }
     | |_- in this expansion of `#[derive(Ord)]`
     |
note: required because of the requirements on the impl of `Ord` for `WithOptConstParam<rustc_span::def_id::DefId>`
    --> compiler/rustc_middle/src/ty/mod.rs:1136:37
     |
1136 |   #[derive(PartialEq, Eq, PartialOrd, Ord)]
     |                                       |
     |                                       in this derive macro expansion
     |                                       in this derive macro expansion
     |
     |
    ::: /checkout/library/core/src/cmp.rs:832:1
     |
832  |   pub macro Ord($item:item) {
     | |_|
     | |
833  | |     /* compiler built-in */
834  | | }
834  | | }
     | | -
     | |_|
     | |_in this expansion of `#[derive(Ord)]`
     |   in this expansion of `#[derive(Ord)]`

error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/ty/instance.rs:42:15
    |
26  |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    |                                                    --- in this derive macro expansion
42  |       Intrinsic(DefId),
42  |       Intrinsic(DefId),
    |                 ^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
   ::: /checkout/library/core/src/cmp.rs:832:1
    |
    |
832 | / pub macro Ord($item:item) {
834 | | }
834 | | }
    | |_- in this expansion of `#[derive(Ord)]`

error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/ty/instance.rs:49:16
    |
26  |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    |                                                    --- in this derive macro expansion
...
49  |       VtableShim(DefId),
    |                  ^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
   ::: /checkout/library/core/src/cmp.rs:832:1
    |
    |
832 | / pub macro Ord($item:item) {
834 | | }
834 | | }
    | |_- in this expansion of `#[derive(Ord)]`

error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/ty/instance.rs:62:15
    |
26  |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    |                                                    --- in this derive macro expansion
...
62  |       ReifyShim(DefId),
    |                 ^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
   ::: /checkout/library/core/src/cmp.rs:832:1
    |
    |
832 | / pub macro Ord($item:item) {
834 | | }
834 | | }
    | |_- in this expansion of `#[derive(Ord)]`

error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/ty/instance.rs:67:15
    |
26  |   #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    |                                                    --- in this derive macro expansion
...
