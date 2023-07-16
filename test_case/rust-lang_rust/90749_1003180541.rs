plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
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

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_middle` due to 2 previous errors
