plain
#################################                                         46.6%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-11-30/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
    Updating git repository `https://github.com/cuviper/indexmap.git`
---
   Compiling indexmap v1.7.0
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
   Compiling crossbeam-epoch v0.8.2
   Compiling indexmap v1.7.0 (https://github.com/cuviper/indexmap.git?branch=rustc-rayon#1de6f77c)
   Compiling unicode-normalization v0.1.13
   Compiling unic-emoji-char v0.9.0
   Compiling psm v0.1.16
   Compiling stacker v0.1.14
---
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>: Encodable<__E>` is not satisfied
   |
   |
51 |   #[derive(Default, TyEncodable, TyDecodable, Debug, HashStable)]
   |                     |
   |                     |
   |                     the trait `Encodable<__E>` is not implemented for `indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>`
   |                     in this derive macro expansion
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
   |
94 | /         pub fn $derives(
95 | |             i: $crate::macros::TokenStream
95 | |             i: $crate::macros::TokenStream
96 | |         ) -> $crate::macros::TokenStream {
   | |________________________________________- in this expansion of `#[derive(TyEncodable)]`

error[E0277]: the trait bound `indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>: Decodable<__D>` is not satisfied
   |
   |
51 |   #[derive(Default, TyEncodable, TyDecodable, Debug, HashStable)]
   |                                  |
   |                                  |
   |                                  the trait `Decodable<__D>` is not implemented for `indexmap::map::IndexMap<SimplifiedTypeGen<rustc_span::def_id::DefId>, Vec<rustc_span::def_id::DefId>, BuildHasherDefault<FxHasher>>`
   |                                  in this derive macro expansion
  ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
   |
94 | /         pub fn $derives(
95 | |             i: $crate::macros::TokenStream
95 | |             i: $crate::macros::TokenStream
96 | |         ) -> $crate::macros::TokenStream {
   | |________________________________________- in this expansion of `#[derive(TyDecodable)]`

error[E0277]: the trait bound `indexmap::map::IndexMap<rustc_hir::HirId, Vec<CapturedPlace<'_>>, BuildHasherDefault<FxHasher>>: Encodable<__E>` is not satisfied
   --> compiler/rustc_middle/src/ty/context.rs:339:10
    |
339 |   #[derive(TyEncodable, TyDecodable, Debug)]
    |            |
    |            |
    |            the trait `Encodable<__E>` is not implemented for `indexmap::map::IndexMap<rustc_hir::HirId, Vec<CapturedPlace<'_>>, BuildHasherDefault<FxHasher>>`
    |            in this derive macro expansion
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(TyEncodable)]`
    |
    = note: required because of the requirements on the impl of `Encodable<__E>` for `HashMap<rustc_span::def_id::DefId, indexmap::map::IndexMap<rustc_hir::HirId, Vec<CapturedPlace<'_>>, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`

error[E0277]: the trait bound `indexmap::map::IndexMap<rustc_hir::HirId, Vec<CapturedPlace<'_>>, BuildHasherDefault<FxHasher>>: Decodable<__D>` is not satisfied
   --> compiler/rustc_middle/src/ty/context.rs:339:23
    |
339 |   #[derive(TyEncodable, TyDecodable, Debug)]
    |                         |
    |                         |
    |                         the trait `Decodable<__D>` is not implemented for `indexmap::map::IndexMap<rustc_hir::HirId, Vec<CapturedPlace<'_>>, BuildHasherDefault<FxHasher>>`
    |                         in this derive macro expansion
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.6/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of `#[derive(TyDecodable)]`
    |
    = note: required because of the requirements on the impl of `Decodable<__D>` for `HashMap<rustc_span::def_id::DefId, indexmap::map::IndexMap<rustc_hir::HirId, Vec<CapturedPlace<'_>>, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
