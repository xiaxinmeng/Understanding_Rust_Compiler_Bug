plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `std::cell::UnsafeCell<Option<Vec<mir::BasicBlock>>>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/hir/map/mod.rs:536:61
     |
536  |                 par_iter(owner_info.nodes.bodies.range(..)).for_each(|(local_id, _)| {
     |                                                             ^^^^^^^^ `std::cell::UnsafeCell<Option<Vec<mir::BasicBlock>>>` cannot be shared between threads safely
     |
     = help: within `mir::Body<'hir>`, the trait `std::marker::Sync` is not implemented for `std::cell::UnsafeCell<Option<Vec<mir::BasicBlock>>>`
     = note: required because it appears within the type `std::lazy::OnceCell<Vec<mir::BasicBlock>>`
note: required because it appears within the type `PostorderCache`
     |
     |
315  | pub(super) struct PostorderCache {
     |                   ^^^^^^^^^^^^^^
note: required because it appears within the type `mir::Body<'hir>`
     |
254  | pub struct Body<'tcx> {
     |            ^^^^
     |            ^^^^
     = note: required because of the requirements on the impl of `Send` for `&'hir mir::Body<'hir>`
     = note: required because it appears within the type `(&'hir mir::Body<'hir>, DepNodeIndex)`
     = note: required because it appears within the type `(rustc_span::def_id::DefId, (&'hir mir::Body<'hir>, DepNodeIndex))`
     = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (&'hir mir::Body<'hir>, DepNodeIndex))>`
     = note: required because it appears within the type `hashbrown::map::HashMap<rustc_span::def_id::DefId, (&'hir mir::Body<'hir>, DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<rustc_span::def_id::DefId, (&'hir mir::Body<'hir>, DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, HashMap<rustc_span::def_id::DefId, (&'hir mir::Body<'hir>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::Lock<HashMap<rustc_span::def_id::DefId, (&'hir mir::Body<'hir>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<rustc_span::def_id::DefId, (&'hir mir::Body<'hir>, DepNodeIndex), BuildHasherDefault<FxHasher>>>>`
     = note: required because it appears within the type `[sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<rustc_span::def_id::DefId, (&'hir mir::Body<'hir>, DepNodeIndex), BuildHasherDefault<FxHasher>>>>; 32]`
     = note: required because it appears within the type `Sharded<HashMap<rustc_span::def_id::DefId, (&'hir mir::Body<'hir>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &'hir mir::Body<'hir>>`
note: required because it appears within the type `QueryCaches<'hir>`
    --> compiler/rustc_middle/src/ty/query.rs:212:20
175  | /  macro_rules! define_callbacks {
175  | /  macro_rules! define_callbacks {
176  | |      (<$tcx:tt>
177  | |       $($(#[$attr:meta])*
178  | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
212  | |          pub struct QueryCaches<$tcx> {
...    |
320  | |      };
321  | |  }
     | |__- in this expansion of `define_callbacks!` (#2)
---
     |  | -
     |  | |
     |  |_in this expansion of `rustc_query_append!` (#1)
     |    in this macro invocation (#2)
note: required because it appears within the type `GlobalCtxt<'hir>`
    --> compiler/rustc_middle/src/ty/context.rs:1000:12
1000 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&'hir GlobalCtxt<'hir>`
note: required because it appears within the type `context::TyCtxt<'hir>`
    --> compiler/rustc_middle/src/ty/context.rs:988:12
988  | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
note: required because it appears within the type `map::Map<'hir>`
    --> compiler/rustc_middle/src/hir/map/mod.rs:74:12
     |
74   | pub struct Map<'hir> {
     |            ^^^
     = note: required because it appears within the type `&map::Map<'hir>`
     = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/map/mod.rs:536:70: 540:18]`
note: required by a bound in `rustc_data_structures::sync::ParallelIterator::for_each`
     |
     |
360  |         OP: Fn(Self::Item) + Sync + Send,
     |                              ^^^^ required by this bound in `rustc_data_structures::sync::ParallelIterator::for_each`

error[E0277]: `std::cell::UnsafeCell<Option<Vec<mir::BasicBlock>>>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/hir/map/mod.rs:698:45
     |
698  |             par_iter(&items.submodules[..]).for_each(|&sm| par_iter_submodules(tcx, sm, f));
     |                                             ^^^^^^^^ `std::cell::UnsafeCell<Option<Vec<mir::BasicBlock>>>` cannot be shared between threads safely
     |
     = help: within `mir::Body<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::UnsafeCell<Option<Vec<mir::BasicBlock>>>`
     = note: required because it appears within the type `std::lazy::OnceCell<Vec<mir::BasicBlock>>`
note: required because it appears within the type `PostorderCache`
     |
     |
315  | pub(super) struct PostorderCache {
note: required because it appears within the type `mir::Body<'_>`
    --> compiler/rustc_middle/src/mir/mod.rs:254:12
     |
254  | pub struct Body<'tcx> {
254  | pub struct Body<'tcx> {
     |            ^^^^
     = note: required because of the requirements on the impl of `Send` for `&mir::Body<'_>`
     = note: required because it appears within the type `(&mir::Body<'_>, DepNodeIndex)`
     = note: required because it appears within the type `(rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex))`
     = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex))>`
     = note: required because it appears within the type `hashbrown::map::HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::Lock<HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>>>`
     = note: required because it appears within the type `[sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>>>; 32]`
     = note: required because it appears within the type `Sharded<HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &mir::Body<'_>>`
note: required because it appears within the type `QueryCaches<'_>`
    --> compiler/rustc_middle/src/ty/query.rs:212:20
175  | /  macro_rules! define_callbacks {
175  | /  macro_rules! define_callbacks {
176  | |      (<$tcx:tt>
177  | |       $($(#[$attr:meta])*
178  | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
212  | |          pub struct QueryCaches<$tcx> {
...    |
320  | |      };
321  | |  }
     | |__- in this expansion of `define_callbacks!` (#2)
---
    --> compiler/rustc_middle/src/ty/context.rs:1000:12
     |
1000 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:988:12
     |
988  | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
     = note: required because it appears within the type `&context::TyCtxt<'_>`
     = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/map/mod.rs:698:54: 698:91]`
note: required by a bound in `rustc_data_structures::sync::ParallelIterator::for_each`
     |
     |
360  |         OP: Fn(Self::Item) + Sync + Send,
     |                              ^^^^ required by this bound in `rustc_data_structures::sync::ParallelIterator::for_each`

error[E0277]: `std::cell::UnsafeCell<Option<Vec<mir::BasicBlock>>>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/ty/context.rs:1833:13
     |
1833 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::UnsafeCell<Option<Vec<mir::BasicBlock>>>` cannot be shared between threads safely
     |
     = help: within `mir::Body<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::UnsafeCell<Option<Vec<mir::BasicBlock>>>`
     = note: required because it appears within the type `std::lazy::OnceCell<Vec<mir::BasicBlock>>`
note: required because it appears within the type `PostorderCache`
     |
     |
315  | pub(super) struct PostorderCache {
note: required because it appears within the type `mir::Body<'_>`
    --> compiler/rustc_middle/src/mir/mod.rs:254:12
     |
254  | pub struct Body<'tcx> {
254  | pub struct Body<'tcx> {
     |            ^^^^
     = note: required because of the requirements on the impl of `Send` for `&mir::Body<'_>`
     = note: required because it appears within the type `(&mir::Body<'_>, DepNodeIndex)`
     = note: required because it appears within the type `(rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex))`
     = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex))>`
     = note: required because it appears within the type `hashbrown::map::HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::Lock<HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>>>`
     = note: required because it appears within the type `[sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>>>; 32]`
     = note: required because it appears within the type `Sharded<HashMap<rustc_span::def_id::DefId, (&mir::Body<'_>, DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &mir::Body<'_>>`
note: required because it appears within the type `QueryCaches<'_>`
    --> compiler/rustc_middle/src/ty/query.rs:212:20
175  | /  macro_rules! define_callbacks {
175  | /  macro_rules! define_callbacks {
176  | |      (<$tcx:tt>
177  | |       $($(#[$attr:meta])*
178  | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
212  | |          pub struct QueryCaches<$tcx> {
...    |
320  | |      };
321  | |  }
     | |__- in this expansion of `define_callbacks!` (#2)
---
    --> compiler/rustc_middle/src/ty/context.rs:1000:12
     |
1000 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:988:12
     |
988  | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
note: required because it appears within the type `ImplicitCtxt<'_, '_>`
    --> compiler/rustc_middle/src/ty/context.rs:1737:16
     |
1737 |     pub struct ImplicitCtxt<'a, 'tcx> {
note: required by a bound in `assert_sync`
    --> /checkout/compiler/rustc_data_structures/src/sync.rs:362:32
     |
     |
362  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ^^^^ required by this bound in `assert_sync`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to 3 previous errors
