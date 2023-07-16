plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: the method `hash_stable` exists for reference `&std::sync::Arc<HashSet<rustc_span::def_id::LocalDefId, BuildHasherDefault<FxHasher>>>`, but its trait bounds were not satisfied
   --> compiler/rustc_middle/src/ty/context.rs:807:32
    |
807 |             used_trait_imports.hash_stable(hcx, hasher);
    |                                ^^^^^^^^^^^ method cannot be called on `&std::sync::Arc<HashSet<rustc_span::def_id::LocalDefId, BuildHasherDefault<FxHasher>>>` due to unsatisfied trait bounds
   ::: /checkout/library/std/src/collections/hash/set.rs:112:1
    |
    |
112 | pub struct HashSet<T, S = RandomState> {
    | -------------------------------------- doesn't satisfy `_: HashStable<_>`
   ::: /checkout/library/alloc/src/sync.rs:235:1
    |
    |
235 | pub struct Arc<T: ?Sized> {
    | ------------------------- doesn't satisfy `_: HashStable<_>`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `HashSet<rustc_span::def_id::LocalDefId, BuildHasherDefault<FxHasher>>: HashStable<_>`
            which is required by `std::sync::Arc<HashSet<rustc_span::def_id::LocalDefId, BuildHasherDefault<FxHasher>>>: HashStable<_>`
            `std::sync::Arc<HashSet<rustc_span::def_id::LocalDefId, BuildHasherDefault<FxHasher>>>: HashStable<_>`
            which is required by `&std::sync::Arc<HashSet<rustc_span::def_id::LocalDefId, BuildHasherDefault<FxHasher>>>: HashStable<_>`

error[E0599]: the method `hash_stable` exists for reference `&HashSet<rustc_span::def_id::DefId, BuildHasherDefault<FxHasher>>`, but its trait bounds were not satisfied
   --> compiler/rustc_middle/src/ty/context.rs:809:35
    |
809 |             concrete_opaque_types.hash_stable(hcx, hasher);
    |                                   ^^^^^^^^^^^ method cannot be called on `&HashSet<rustc_span::def_id::DefId, BuildHasherDefault<FxHasher>>` due to unsatisfied trait bounds
   ::: /checkout/library/std/src/collections/hash/set.rs:112:1
    |
    |
112 | pub struct HashSet<T, S = RandomState> {
    | -------------------------------------- doesn't satisfy `_: HashStable<_>`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `HashSet<rustc_span::def_id::DefId, BuildHasherDefault<FxHasher>>: HashStable<_>`
            which is required by `&HashSet<rustc_span::def_id::DefId, BuildHasherDefault<FxHasher>>: HashStable<_>`

error[E0599]: the method `hash_stable` exists for reference `&HashSet<rustc_span::Symbol, BuildHasherDefault<FxHasher>>`, but its trait bounds were not satisfied
    |
9   |       #[derive(HashStable, Debug)]
    |                ^^^^^^^^^^
    |                |
    |                |
    |                method cannot be called on `&HashSet<rustc_span::Symbol, BuildHasherDefault<FxHasher>>` due to unsatisfied trait bounds
    |
   ::: /checkout/library/std/src/collections/hash/set.rs:112:1
    |
    |
112 |   pub struct HashSet<T, S = RandomState> {
    |   -------------------------------------- doesn't satisfy `_: HashStable<_>`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(HashStable)]`
    |
    = note: the following trait bounds were not satisfied:
            `HashSet<rustc_span::Symbol, BuildHasherDefault<FxHasher>>: HashStable<_>`
            which is required by `&HashSet<rustc_span::Symbol, BuildHasherDefault<FxHasher>>: HashStable<_>`

error[E0599]: the method `hash_stable` exists for reference `&HashMap<rustc_span::def_id::LocalDefId, HashSet<rustc_hir::ItemLocalId, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`, but its trait bounds were not satisfied
    |
90  |   #[derive(Default, HashStable, Debug)]
    |                     ^^^^^^^^^^
    |                     |
    |                     |
    |                     method cannot be called on `&HashMap<rustc_span::def_id::LocalDefId, HashSet<rustc_hir::ItemLocalId, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>` due to unsatisfied trait bounds
    |
   ::: /checkout/library/std/src/collections/hash/map.rs:209:1
    |
    |
209 |   pub struct HashMap<K, V, S = RandomState> {
    |   ----------------------------------------- doesn't satisfy `_: HashStable<_>`
   ::: /checkout/library/std/src/collections/hash/set.rs:112:1
    |
    |
112 |   pub struct HashSet<T, S = RandomState> {
    |   -------------------------------------- doesn't satisfy `_: HashStable<_>`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(HashStable)]`
    |
    = note: the following trait bounds were not satisfied:
            `HashSet<rustc_hir::ItemLocalId, BuildHasherDefault<FxHasher>>: HashStable<_>`
            which is required by `HashMap<rustc_span::def_id::LocalDefId, HashSet<rustc_hir::ItemLocalId, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>: HashStable<_>`
            `HashMap<rustc_span::def_id::LocalDefId, HashSet<rustc_hir::ItemLocalId, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>: HashStable<_>`
            which is required by `&HashMap<rustc_span::def_id::LocalDefId, HashSet<rustc_hir::ItemLocalId, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>: HashStable<_>`

error[E0599]: the method `hash_stable` exists for reference `&HashSet<rustc_span::Symbol, BuildHasherDefault<FxHasher>>`, but its trait bounds were not satisfied
    |
59  |   #[derive(HashStable, Debug)]
    |            ^^^^^^^^^^
    |            |
    |            |
    |            method cannot be called on `&HashSet<rustc_span::Symbol, BuildHasherDefault<FxHasher>>` due to unsatisfied trait bounds
    |
   ::: /checkout/library/std/src/collections/hash/set.rs:112:1
    |
    |
112 |   pub struct HashSet<T, S = RandomState> {
    |   -------------------------------------- doesn't satisfy `_: HashStable<_>`
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(HashStable)]`
    |
    = note: the following trait bounds were not satisfied:
            `HashSet<rustc_span::Symbol, BuildHasherDefault<FxHasher>>: HashStable<_>`
            which is required by `&HashSet<rustc_span::Symbol, BuildHasherDefault<FxHasher>>: HashStable<_>`

error[E0277]: the trait bound `StableSet<rustc_hir::ItemLocalId>: Encodable<__E>` is not satisfied
   --> compiler/rustc_middle/src/ty/context.rs:351:10
    |
351 |   #[derive(TyEncodable, TyDecodable, Debug)]
    |            |
    |            |
    |            the trait `Encodable<__E>` is not implemented for `StableSet<rustc_hir::ItemLocalId>`
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(TyEncodable)]`

error[E0277]: the trait bound `StableSet<rustc_hir::ItemLocalId>: Decodable<__D>` is not satisfied
   --> compiler/rustc_middle/src/ty/context.rs:459:5
    |
459 |     coercion_casts: ItemLocalSet,
    |     ^^^^^^^^^^^^^^ the trait `Decodable<__D>` is not implemented for `StableSet<rustc_hir::ItemLocalId>`

error[E0277]: the trait bound `StableSet<rustc_hir::ItemLocalId>: Decodable<__D>` is not satisfied
   --> compiler/rustc_middle/src/ty/context.rs:351:23
    |
351 |   #[derive(TyEncodable, TyDecodable, Debug)]
    |                         |
    |                         |
    |                         the trait `Decodable<__D>` is not implemented for `StableSet<rustc_hir::ItemLocalId>`
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(TyDecodable)]`

error[E0277]: the trait bound `StableSet<rustc_hir::ItemLocalId>: Decodable<__D>` is not satisfied
   --> compiler/rustc_middle/src/ty/context.rs:511:9
    |
511 |     pub treat_byte_string_as_slice: ItemLocalSet,
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Decodable<__D>` is not implemented for `StableSet<rustc_hir::ItemLocalId>`
Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 9 previous errors
warning: build failed, waiting for other jobs to finish...
