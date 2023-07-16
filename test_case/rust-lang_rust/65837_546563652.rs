plain
2019-10-26T01:13:25.0008984Z    Compiling syntax_expand v0.0.0 (/checkout/src/libsyntax_expand)
2019-10-26T01:14:37.8868601Z [RUSTC-TIMING] syntax_expand test:false 72.882
2019-10-26T01:14:38.8611626Z [RUSTC-TIMING] syntax test:false 85.090
2019-10-26T01:14:38.8668543Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-26T01:14:50.7413190Z error[E0277]: the trait bound `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>: rustc_data_structures::sync::ParallelIterator` is not satisfied
2019-10-26T01:14:50.7414902Z    --> src/librustc/hir/map/hir_id_validator.rs:11:14
2019-10-26T01:14:50.7415448Z     |
2019-10-26T01:14:50.7416069Z 11  |     par_iter(hir_map.map.iter_enumerated()).for_each(|(owner, local_map)| {
2019-10-26T01:14:50.7417239Z     |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `rustc_data_structures::sync::ParallelIterator` is not implemented for `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>`
2019-10-26T01:14:50.7417919Z     |
2019-10-26T01:14:50.7418810Z     = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>`
2019-10-26T01:14:50.7423790Z 
2019-10-26T01:14:50.7526759Z error[E0277]: the trait bound `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>: rustc_data_structures::sync::ParallelIterator` is not satisfied
2019-10-26T01:14:50.7527758Z   --> src/librustc/hir/map/hir_id_validator.rs:11:5
2019-10-26T01:14:50.7528726Z    |
2019-10-26T01:14:50.7529471Z 11 |     par_iter(hir_map.map.iter_enumerated()).for_each(|(owner, local_map)| {
2019-10-26T01:14:50.7530332Z    |     ^^^^^^^^ the trait `rustc_data_structures::sync::ParallelIterator` is not implemented for `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>`
2019-10-26T01:14:50.7530848Z    |
2019-10-26T01:14:50.7531610Z    = note: required because of the requirements on the impl of `rustc_rayon::iter::IntoParallelIterator` for `std::iter::Map<std::iter::Enumerate<std::slice::Iter<'_, rustc_index::vec::IndexVec<hir::item_local_id_inner::ItemLocalId, std::option::Option<hir::map::Entry<'_>>>>>, rustc_index::vec::IntoIdx<hir::def_id::DefIndex>>`
2019-10-26T01:15:05.2849234Z error: unused import: `ParallelIterator`
2019-10-26T01:15:05.2849234Z error: unused import: `ParallelIterator`
2019-10-26T01:15:05.2850529Z  --> src/librustc/hir/map/hir_id_validator.rs:3:41
2019-10-26T01:15:05.2851056Z   |
2019-10-26T01:15:05.2851651Z 3 | use rustc_data_structures::sync::{Lock, ParallelIterator, par_iter};
2019-10-26T01:15:05.2852737Z   |
2019-10-26T01:15:05.2853672Z   = note: `-D unused-imports` implied by `-D warnings`
2019-10-26T01:15:05.2876220Z 
2019-10-26T01:15:05.2887385Z error: aborting due to 3 previous errors
---
2019-10-26T01:15:16.5437255Z   local time: Sat Oct 26 01:15:16 UTC 2019
2019-10-26T01:15:17.3598015Z   network time: Sat, 26 Oct 2019 01:15:17 GMT
2019-10-26T01:15:17.3598220Z == end clock drift check ==
2019-10-26T01:15:18.2154883Z 
2019-10-26T01:15:18.2280577Z ##[error]Bash exited with code '1'.
2019-10-26T01:15:18.2324160Z ##[section]Starting: Upload CPU usage statistics
2019-10-26T01:15:18.2332183Z ==============================================================================
2019-10-26T01:15:18.2332303Z Task         : Bash
2019-10-26T01:15:18.2332382Z Description  : Run a Bash script on macOS, Linux, or Windows
