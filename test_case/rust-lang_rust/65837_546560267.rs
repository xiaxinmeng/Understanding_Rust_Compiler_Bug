plain
2019-10-26T00:50:07.3511643Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-26T00:50:07.3760708Z ##[command]git config gc.auto 0
2019-10-26T00:50:07.3838654Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-26T00:50:07.3900833Z ##[command]git config --get-all http.proxy
2019-10-26T00:50:07.4024218Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65837/merge:refs/remotes/pull/65837/merge
---
2019-10-26T00:57:03.9649251Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-26T00:57:05.1524561Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-26T00:57:15.1128118Z     Checking syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-10-26T00:57:16.7363573Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-26T00:57:26.7787382Z error[E0277]: the trait bound `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>: rustc_data_structures::sync::ParallelIterator` is not satisfied
2019-10-26T00:57:26.7788495Z    --> src/librustc/hir/map/hir_id_validator.rs:11:14
2019-10-26T00:57:26.7789073Z     |
2019-10-26T00:57:26.7789853Z 11  |     par_iter(hir_map.map.iter_enumerated()).for_each(|(owner, local_map)| {
2019-10-26T00:57:26.7790999Z     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::sync::ParallelIterator` is not implemented for `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>`
2019-10-26T00:57:26.7791939Z    ::: /checkout/src/librustc_data_structures/sync.rs:394:28
2019-10-26T00:57:26.7792548Z     |
2019-10-26T00:57:26.7792548Z     |
2019-10-26T00:57:26.7793068Z 394 |         pub fn par_iter<T: IntoParallelIterator>(t: T) -> T::Iter {
2019-10-26T00:57:26.7793858Z     |                            -------------------- required by this bound in `rustc_data_structures::sync::par_iter`
2019-10-26T00:57:26.7795977Z     |
2019-10-26T00:57:26.7797400Z     = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>`
2019-10-26T00:57:26.7797745Z 
2019-10-26T00:57:26.7895005Z error[E0277]: the trait bound `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>: rustc_data_structures::sync::ParallelIterator` is not satisfied
2019-10-26T00:57:26.7897147Z   --> src/librustc/hir/map/hir_id_validator.rs:11:5
2019-10-26T00:57:26.7897765Z    |
2019-10-26T00:57:26.7898314Z 11 |     par_iter(hir_map.map.iter_enumerated()).for_each(|(owner, local_map)| {
2019-10-26T00:57:26.7899092Z    |     ^^^^^^^^ the trait `rustc_data_structures::sync::ParallelIterator` is not implemented for `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>`
2019-10-26T00:57:26.7899527Z    |
2019-10-26T00:57:26.7900337Z    = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>`
2019-10-26T00:57:38.5995825Z error: unused import: `ParallelIterator`
2019-10-26T00:57:38.5995825Z error: unused import: `ParallelIterator`
2019-10-26T00:57:38.5996755Z  --> src/librustc/hir/map/hir_id_validator.rs:3:41
2019-10-26T00:57:38.5997204Z   |
2019-10-26T00:57:38.5997749Z 3 | use rustc_data_structures::sync::{Lock, ParallelIterator, par_iter};
2019-10-26T00:57:38.5998934Z   |
2019-10-26T00:57:38.5999334Z   = note: `-D unused-imports` implied by `-D warnings`
2019-10-26T00:57:38.5999501Z 
2019-10-26T00:57:38.6026719Z error: aborting due to 3 previous errors
---
2019-10-26T00:57:38.7733931Z   local time: Sat Oct 26 00:57:38 UTC 2019
2019-10-26T00:57:38.8508407Z   network time: Sat, 26 Oct 2019 00:57:38 GMT
2019-10-26T00:57:38.8512566Z == end clock drift check ==
2019-10-26T00:57:39.5349951Z 
2019-10-26T00:57:39.5442680Z ##[error]Bash exited with code '1'.
2019-10-26T00:57:39.5472542Z ##[section]Starting: Checkout
2019-10-26T00:57:39.5473950Z ==============================================================================
2019-10-26T00:57:39.5473992Z Task         : Get sources
2019-10-26T00:57:39.5474046Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
