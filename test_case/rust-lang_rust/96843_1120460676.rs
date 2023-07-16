plain

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-04-05/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
Building rustbuild
    Updating git repository `https://github.com/nbdd0121/stacker.git`
    Updating crates.io index
    Updating git repository `https://github.com/nbdd0121/stackful.git`
---
    Checking ena v0.14.0
    Checking polonius-engine v0.13.0
    Checking tracing-log v0.1.2
    Checking rustc_lexer v0.1.0 (/checkout/compiler/rustc_lexer)
   Compiling psm v0.1.18 (https://github.com/nbdd0121/stacker.git#eae7e163)
   Compiling stacker v0.1.14 (https://github.com/nbdd0121/stacker.git#eae7e163)
   Compiling stackful v0.1.3 (https://github.com/nbdd0121/stackful.git?branch=dev#5fedf72a)
    Checking quote v1.0.18
    Checking aho-corasick v0.7.18
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
    Checking unicode-security v0.0.5
---
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)` cannot be sent between threads safely
    --> compiler/rustc_middle/src/hir/map/mod.rs:531:61
     |
531  |                 par_iter(owner_info.nodes.bodies.range(..)).for_each(|(local_id, _)| {
     |                                                             ^^^^^^^^ `(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)` cannot be sent between threads safely
     |
     = help: the trait `Send` is not implemented for `(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)`
     = note: required because of the requirements on the impl of `Send` for `Unique<(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)>`
     = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)>`
     = note: required because it appears within the type `std::pin::Pin<std::boxed::Box<(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)>>`
     = note: required because it appears within the type `std::mem::ManuallyDrop<std::pin::Pin<std::boxed::Box<(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)>>>`
note: required because it appears within the type `TypeckResultGenerator<'hir>`
    --> compiler/rustc_middle/src/ty/context.rs:380:12
     |
380  | pub struct TypeckResultGenerator<'tcx>(
     |            ^^^^^^^^^^^^^^^^^^^^^
     = note: required because it appears within the type `Option<TypeckResultGenerator<'hir>>`
     = note: required because of the requirements on the impl of `Send` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, Option<TypeckResultGenerator<'hir>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::RwLock<Option<TypeckResultGenerator<'hir>>>`
     = note: required because it appears within the type `rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>`
     = note: required because of the requirements on the impl of `Send` for `TypedArena<rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>>`
note: required because it appears within the type `Arena<'hir>`
    --> /checkout/compiler/rustc_arena/src/lib.rs:552:16
     |
550  | /  pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
552  | |      pub struct Arena<'tcx> {
     | |                 ^^^^^
     | |                 ^^^^^
553  | |          pub dropless: $crate::DroplessArena,
625  | |      }
626  | |  }
626  | |  }
     | |__- in this expansion of `rustc_arena::declare_arena!` (#2)
    ::: compiler/rustc_middle/src/arena.rs:6:1
     |
     |
6    |  / macro_rules! arena_types {
7    |  |     ($macro:path) => (
8    |  |         $macro!([
     |  |________________-
9    | ||             [] layout: rustc_target::abi::LayoutS<'tcx>,
10   | ||             [] fn_abi: rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
11   | ||             // AdtDef are interned and compared by address
...    ||
106  | ||             [] steal_typeck_generator: rustc_data_structures::steal::Steal<rustc_middle::ty::TypeckResultGenerator<'tcx>>,
     | ||__________- in this macro invocation (#2)
108  |  |     )
109  |  | }
109  |  | }
     |  |_- in this expansion of `arena_types!` (#1)
110  | 
111  |    arena_types!(rustc_arena::declare_arena);
     |    ---------------------------------------- in this macro invocation (#1)
     = note: required because of the requirements on the impl of `std::marker::Sync` for `WorkerLocal<Arena<'hir>>`
     = note: required because it appears within the type `&'hir WorkerLocal<Arena<'hir>>`
note: required because it appears within the type `GlobalCtxt<'hir>`
    --> compiler/rustc_middle/src/ty/context.rs:1020:12
1020 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&'hir GlobalCtxt<'hir>`
note: required because it appears within the type `context::TyCtxt<'hir>`
    --> compiler/rustc_middle/src/ty/context.rs:1008:12
1008 | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
note: required because it appears within the type `map::Map<'hir>`
    --> compiler/rustc_middle/src/hir/map/mod.rs:74:12
     |
74   | pub struct Map<'hir> {
     |            ^^^
     = note: required because it appears within the type `&map::Map<'hir>`
     = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/map/mod.rs:531:70: 535:18]`
note: required by a bound in `rustc_data_structures::sync::ParallelIterator::for_each`
     |
     |
360  |         OP: Fn(Self::Item) + Sync + Send,
     |                              ^^^^ required by this bound in `rustc_data_structures::sync::ParallelIterator::for_each`

error[E0277]: `(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)` cannot be shared between threads safely
    --> compiler/rustc_middle/src/hir/map/mod.rs:531:61
     |
531  |                 par_iter(owner_info.nodes.bodies.range(..)).for_each(|(local_id, _)| {
     |                                                             ^^^^^^^^ `(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)` cannot be shared between threads safely
     |
     = help: the trait `std::marker::Sync` is not implemented for `(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `Unique<(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)>`
     = note: required because it appears within the type `std::boxed::Box<(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)>`
     = note: required because it appears within the type `std::pin::Pin<std::boxed::Box<(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)>>`
     = note: required because it appears within the type `std::mem::ManuallyDrop<std::pin::Pin<std::boxed::Box<(dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &'hir TypeckResults<'hir>> + 'hir)>>>`
note: required because it appears within the type `TypeckResultGenerator<'hir>`
    --> compiler/rustc_middle/src/ty/context.rs:380:12
     |
380  | pub struct TypeckResultGenerator<'tcx>(
     |            ^^^^^^^^^^^^^^^^^^^^^
     = note: required because it appears within the type `Option<TypeckResultGenerator<'hir>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, Option<TypeckResultGenerator<'hir>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::RwLock<Option<TypeckResultGenerator<'hir>>>`
     = note: required because it appears within the type `rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>`
     = note: required because of the requirements on the impl of `Send` for `&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>`
     = note: required because it appears within the type `(&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>)`
     = note: required because it appears within the type `((&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>), DepNodeIndex)`
     = note: required because it appears within the type `((rustc_span::def_id::LocalDefId, u32), ((&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>), DepNodeIndex))`
     = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<((rustc_span::def_id::LocalDefId, u32), ((&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>), DepNodeIndex))>`
     = note: required because it appears within the type `hashbrown::map::HashMap<(rustc_span::def_id::LocalDefId, u32), ((&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>), DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<(rustc_span::def_id::LocalDefId, u32), ((&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>), DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, HashMap<(rustc_span::def_id::LocalDefId, u32), ((&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::Lock<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>>`
     = note: required because it appears within the type `[sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>>; 32]`
     = note: required because it appears within the type `Sharded<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_query_system::query::caches::DefaultCache<(rustc_span::def_id::LocalDefId, u32), (&'hir rustc_data_structures::steal::Steal<TypeckResultGenerator<'hir>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &'hir TypeckResults<'hir>>)>`
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
    --> compiler/rustc_middle/src/ty/context.rs:1020:12
1020 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&'hir GlobalCtxt<'hir>`
note: required because it appears within the type `context::TyCtxt<'hir>`
    --> compiler/rustc_middle/src/ty/context.rs:1008:12
1008 | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
note: required because it appears within the type `map::Map<'hir>`
    --> compiler/rustc_middle/src/hir/map/mod.rs:74:12
     |
74   | pub struct Map<'hir> {
     |            ^^^
     = note: required because it appears within the type `&map::Map<'hir>`
     = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/map/mod.rs:531:70: 535:18]`
note: required by a bound in `rustc_data_structures::sync::ParallelIterator::for_each`
     |
     |
360  |         OP: Fn(Self::Item) + Sync + Send,
     |                              ^^^^ required by this bound in `rustc_data_structures::sync::ParallelIterator::for_each`

error[E0277]: `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>` cannot be sent between threads safely
    --> compiler/rustc_middle/src/hir/map/mod.rs:693:45
     |
693  |             par_iter(&items.submodules[..]).for_each(|&sm| par_iter_submodules(tcx, sm, f));
     |                                             ^^^^^^^^ `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>` cannot be sent between threads safely
     |
     = help: the trait `Send` is not implemented for `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>`
     = note: required because of the requirements on the impl of `Send` for `Unique<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>`
     = note: required because it appears within the type `std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>`
     = note: required because it appears within the type `std::pin::Pin<std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>>`
     = note: required because it appears within the type `std::mem::ManuallyDrop<std::pin::Pin<std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>>>`
note: required because it appears within the type `TypeckResultGenerator<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:380:12
     |
380  | pub struct TypeckResultGenerator<'tcx>(
     |            ^^^^^^^^^^^^^^^^^^^^^
     = note: required because it appears within the type `Option<TypeckResultGenerator<'_>>`
     = note: required because of the requirements on the impl of `Send` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, Option<TypeckResultGenerator<'_>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::RwLock<Option<TypeckResultGenerator<'_>>>`
     = note: required because it appears within the type `rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>`
     = note: required because of the requirements on the impl of `Send` for `TypedArena<rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>>`
note: required because it appears within the type `Arena<'_>`
    --> /checkout/compiler/rustc_arena/src/lib.rs:552:16
     |
550  | /  pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
552  | |      pub struct Arena<'tcx> {
     | |                 ^^^^^
     | |                 ^^^^^
553  | |          pub dropless: $crate::DroplessArena,
625  | |      }
626  | |  }
626  | |  }
     | |__- in this expansion of `rustc_arena::declare_arena!` (#2)
    ::: compiler/rustc_middle/src/arena.rs:6:1
     |
     |
6    |  / macro_rules! arena_types {
7    |  |     ($macro:path) => (
8    |  |         $macro!([
     |  |________________-
9    | ||             [] layout: rustc_target::abi::LayoutS<'tcx>,
10   | ||             [] fn_abi: rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
11   | ||             // AdtDef are interned and compared by address
...    ||
106  | ||             [] steal_typeck_generator: rustc_data_structures::steal::Steal<rustc_middle::ty::TypeckResultGenerator<'tcx>>,
     | ||__________- in this macro invocation (#2)
108  |  |     )
109  |  | }
109  |  | }
     |  |_- in this expansion of `arena_types!` (#1)
110  | 
111  |    arena_types!(rustc_arena::declare_arena);
     |    ---------------------------------------- in this macro invocation (#1)
     = note: required because of the requirements on the impl of `std::marker::Sync` for `WorkerLocal<Arena<'_>>`
     = note: required because it appears within the type `&WorkerLocal<Arena<'_>>`
note: required because it appears within the type `GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:1020:12
1020 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:1008:12
     |
1008 | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
     = note: required because it appears within the type `&context::TyCtxt<'_>`
     = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/map/mod.rs:693:54: 693:91]`
note: required by a bound in `rustc_data_structures::sync::ParallelIterator::for_each`
     |
     |
360  |         OP: Fn(Self::Item) + Sync + Send,
     |                              ^^^^ required by this bound in `rustc_data_structures::sync::ParallelIterator::for_each`

error[E0277]: `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/hir/map/mod.rs:693:45
     |
693  |             par_iter(&items.submodules[..]).for_each(|&sm| par_iter_submodules(tcx, sm, f));
     |                                             ^^^^^^^^ `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>` cannot be shared between threads safely
     |
     = help: the trait `std::marker::Sync` is not implemented for `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `Unique<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>`
     = note: required because it appears within the type `std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>`
     = note: required because it appears within the type `std::pin::Pin<std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>>`
     = note: required because it appears within the type `std::mem::ManuallyDrop<std::pin::Pin<std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>>>`
note: required because it appears within the type `TypeckResultGenerator<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:380:12
     |
380  | pub struct TypeckResultGenerator<'tcx>(
     |            ^^^^^^^^^^^^^^^^^^^^^
     = note: required because it appears within the type `Option<TypeckResultGenerator<'_>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, Option<TypeckResultGenerator<'_>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::RwLock<Option<TypeckResultGenerator<'_>>>`
     = note: required because it appears within the type `rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>`
     = note: required because of the requirements on the impl of `Send` for `&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>`
     = note: required because it appears within the type `(&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>)`
     = note: required because it appears within the type `((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex)`
     = note: required because it appears within the type `((rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex))`
     = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<((rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex))>`
     = note: required because it appears within the type `hashbrown::map::HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::Lock<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>>`
     = note: required because it appears within the type `[sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>>; 32]`
     = note: required because it appears within the type `Sharded<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_query_system::query::caches::DefaultCache<(rustc_span::def_id::LocalDefId, u32), (&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>)>`
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
    --> compiler/rustc_middle/src/ty/context.rs:1020:12
     |
1020 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:1008:12
     |
1008 | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
     = note: required because it appears within the type `&context::TyCtxt<'_>`
     = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/map/mod.rs:693:54: 693:91]`
note: required by a bound in `rustc_data_structures::sync::ParallelIterator::for_each`
     |
     |
360  |         OP: Fn(Self::Item) + Sync + Send,
     |                              ^^^^ required by this bound in `rustc_data_structures::sync::ParallelIterator::for_each`

error[E0277]: `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>` cannot be sent between threads safely
    --> compiler/rustc_middle/src/ty/context.rs:1853:13
     |
1853 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>` cannot be sent between threads safely
     |
     = help: the trait `Send` is not implemented for `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>`
     = note: required because of the requirements on the impl of `Send` for `Unique<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>`
     = note: required because it appears within the type `std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>`
     = note: required because it appears within the type `std::pin::Pin<std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>>`
     = note: required because it appears within the type `std::mem::ManuallyDrop<std::pin::Pin<std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>>>`
note: required because it appears within the type `TypeckResultGenerator<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:380:12
     |
380  | pub struct TypeckResultGenerator<'tcx>(
     |            ^^^^^^^^^^^^^^^^^^^^^
     = note: required because it appears within the type `Option<TypeckResultGenerator<'_>>`
     = note: required because of the requirements on the impl of `Send` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, Option<TypeckResultGenerator<'_>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::RwLock<Option<TypeckResultGenerator<'_>>>`
     = note: required because it appears within the type `rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>`
     = note: required because of the requirements on the impl of `Send` for `TypedArena<rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>>`
note: required because it appears within the type `Arena<'_>`
    --> /checkout/compiler/rustc_arena/src/lib.rs:552:16
     |
550  | /  pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
552  | |      pub struct Arena<'tcx> {
     | |                 ^^^^^
     | |                 ^^^^^
553  | |          pub dropless: $crate::DroplessArena,
625  | |      }
626  | |  }
626  | |  }
     | |__- in this expansion of `rustc_arena::declare_arena!` (#2)
    ::: compiler/rustc_middle/src/arena.rs:6:1
     |
     |
6    |  / macro_rules! arena_types {
7    |  |     ($macro:path) => (
8    |  |         $macro!([
     |  |________________-
9    | ||             [] layout: rustc_target::abi::LayoutS<'tcx>,
10   | ||             [] fn_abi: rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
11   | ||             // AdtDef are interned and compared by address
...    ||
106  | ||             [] steal_typeck_generator: rustc_data_structures::steal::Steal<rustc_middle::ty::TypeckResultGenerator<'tcx>>,
     | ||__________- in this macro invocation (#2)
108  |  |     )
109  |  | }
109  |  | }
     |  |_- in this expansion of `arena_types!` (#1)
110  | 
111  |    arena_types!(rustc_arena::declare_arena);
     |    ---------------------------------------- in this macro invocation (#1)
     = note: required because of the requirements on the impl of `std::marker::Sync` for `WorkerLocal<Arena<'_>>`
     = note: required because it appears within the type `&WorkerLocal<Arena<'_>>`
note: required because it appears within the type `GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:1020:12
1020 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:1008:12
     |
1008 | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
note: required because it appears within the type `ImplicitCtxt<'_, '_>`
    --> compiler/rustc_middle/src/ty/context.rs:1757:16
     |
1757 |     pub struct ImplicitCtxt<'a, 'tcx> {
note: required by a bound in `assert_sync`
    --> /checkout/compiler/rustc_data_structures/src/sync.rs:362:32
     |
     |
362  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ^^^^ required by this bound in `assert_sync`

error[E0277]: `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>` cannot be shared between threads safely
    --> compiler/rustc_middle/src/ty/context.rs:1853:13
     |
1853 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>` cannot be shared between threads safely
     |
     = help: the trait `std::marker::Sync` is not implemented for `dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `Unique<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>`
     = note: required because it appears within the type `std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>`
     = note: required because it appears within the type `std::pin::Pin<std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>>`
     = note: required because it appears within the type `std::mem::ManuallyDrop<std::pin::Pin<std::boxed::Box<dyn std::ops::Generator<Yield = (rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), Return = &TypeckResults<'_>>>>>`
note: required because it appears within the type `TypeckResultGenerator<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:380:12
     |
380  | pub struct TypeckResultGenerator<'tcx>(
     |            ^^^^^^^^^^^^^^^^^^^^^
     = note: required because it appears within the type `Option<TypeckResultGenerator<'_>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, Option<TypeckResultGenerator<'_>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::RwLock<Option<TypeckResultGenerator<'_>>>`
     = note: required because it appears within the type `rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>`
     = note: required because of the requirements on the impl of `Send` for `&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>`
     = note: required because it appears within the type `(&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>)`
     = note: required because it appears within the type `((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex)`
     = note: required because it appears within the type `((rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex))`
     = note: required because of the requirements on the impl of `Send` for `hashbrown::raw::RawTable<((rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex))>`
     = note: required because it appears within the type `hashbrown::map::HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because it appears within the type `HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>`
     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_data_structures::sync::Lock<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>>`
     = note: required because it appears within the type `[sharded::CacheAligned<rustc_data_structures::sync::Lock<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>>; 32]`
     = note: required because it appears within the type `Sharded<HashMap<(rustc_span::def_id::LocalDefId, u32), ((&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>), DepNodeIndex), BuildHasherDefault<FxHasher>>>`
     = note: required because it appears within the type `rustc_query_system::query::caches::DefaultCache<(rustc_span::def_id::LocalDefId, u32), (&rustc_data_structures::steal::Steal<TypeckResultGenerator<'_>>, std::ops::GeneratorState<(rustc_span::def_id::LocalDefId, rustc_span::def_id::DefId), &TypeckResults<'_>>)>`
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
    --> compiler/rustc_middle/src/ty/context.rs:1020:12
     |
1020 | pub struct GlobalCtxt<'tcx> {
     |            ^^^^^^^^^^
     = note: required because it appears within the type `&GlobalCtxt<'_>`
    --> compiler/rustc_middle/src/ty/context.rs:1008:12
     |
1008 | pub struct TyCtxt<'tcx> {
     |            ^^^^^^
     |            ^^^^^^
note: required because it appears within the type `ImplicitCtxt<'_, '_>`
    --> compiler/rustc_middle/src/ty/context.rs:1757:16
     |
1757 |     pub struct ImplicitCtxt<'a, 'tcx> {
note: required by a bound in `assert_sync`
    --> /checkout/compiler/rustc_data_structures/src/sync.rs:362:32
     |
     |
362  | pub fn assert_sync<T: ?Sized + Sync>() {}
     |                                ^^^^ required by this bound in `assert_sync`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to 6 previous errors
