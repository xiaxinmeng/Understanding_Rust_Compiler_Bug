plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/traits/chalk.rs:77:5
77  |     type DefId = DefId;
77  |     type DefId = DefId;
    |     ^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
    |
note: required by a bound in `chalk_ir::interner::Interner::DefId`
    |
    |
191 |     type DefId: Debug + Copy + Eq + Ord + Hash;
    |                                     ^^^ required by this bound in `chalk_ir::interner::Interner::DefId`

error[E0277]: the trait bound `adt::AdtDef: Ord` is not satisfied
   --> compiler/rustc_middle/src/traits/chalk.rs:78:5
    |
78  |     type InternedAdtId = &'tcx AdtDef;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Ord` is not implemented for `adt::AdtDef`
    |
    = note: required because of the requirements on the impl of `Ord` for `&'tcx adt::AdtDef`
note: required by a bound in `chalk_ir::interner::Interner::InternedAdtId`
    |
    |
194 |     type InternedAdtId: Debug + Copy + Eq + Ord + Hash;
    |                                             ^^^ required by this bound in `chalk_ir::interner::Interner::InternedAdtId`
error[E0308]: mismatched types
   --> compiler/rustc_middle/src/hir/mod.rs:109:49
    |
    |
109 |     providers.all_local_trait_impls = |tcx, ()| &tcx.resolutions(()).trait_impls;
    |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `HashMap`, found struct `BTreeMap`
    |
    = note: expected reference `&HashMap<rustc_span::def_id::DefId, Vec<rustc_span::def_id::LocalDefId>, BuildHasherDefault<FxHasher>>`
               found reference `&BTreeMap<rustc_span::def_id::DefId, Vec<rustc_span::def_id::LocalDefId>>`

error[E0277]: the trait bound `rustc_span::def_id::DefId: Ord` is not satisfied
   --> compiler/rustc_middle/src/ty/print/pretty.rs:886:21
886 |         auto_traits.sort();
886 |         auto_traits.sort();
    |                     ^^^^ the trait `Ord` is not implemented for `rustc_span::def_id::DefId`
    |
    = note: required because of the requirements on the impl of `Ord` for `(std::string::String, rustc_span::def_id::DefId)`

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
   ::: compiler/rustc_middle/src/ty/fast_reject.rs:20:1
    |
    |
20  | / pub enum SimplifiedTypeGen<D>
21  | | where
22  | |     D: Copy + Debug + Eq,
23  | | {
...   |
45  | |     ForeignSimplifiedType(DefId),
46  | | }
    | |_- doesn't satisfy `_: HashStable<_>`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
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
   --> /checkout/compiler/rustc_data_structures/src/stable_hasher.rs:168:1
    |
    |
168 | / pub trait HashStable<CTX> {
169 | |     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher);
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
   ::: compiler/rustc_middle/src/ty/fast_reject.rs:20:1
    |
    |
20  | / pub enum SimplifiedTypeGen<D>
21  | | where
22  | |     D: Copy + Debug + Eq,
23  | | {
...   |
45  | |     ForeignSimplifiedType(DefId),
46  | | }
    | |_- doesn't satisfy `_: HashStable<_>`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
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
   --> /checkout/compiler/rustc_data_structures/src/stable_hasher.rs:168:1
    |
    |
168 | / pub trait HashStable<CTX> {
169 | |     fn hash_stable(&self, hcx: &mut CTX, hasher: &mut StableHasher);
    | |_^

Some errors have detailed explanations: E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
