plain
   Compiling rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
   Compiling rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0412]: cannot find type `FxHashSet` in this scope
     |
18   | / rustc_queries! {
19   | |     query trigger_delay_span_bug(key: DefId) -> () {
20   | |         desc { "trigger a delay span bug" }
20   | |         desc { "trigger a delay span bug" }
21   | |     }
...    |
1049 | |     query asm_target_features(def_id: DefId) -> &'tcx FxHashSet<Symbol> {
     | |                                                       ^^^^^^^^^ help: a type alias with a similar name exists: `FxHashMap`
1946 | |     }
1947 | | }
     | |_- in this expansion of `rustc_query_append!`
     |
     |
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-hash-1.1.0/src/lib.rs:43:1
     |
43   |   pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
     |   ----------------------------------------------------------------------- similarly named type alias `FxHashMap` defined here
    ::: compiler/rustc_middle/src/ty/query.rs:339:1
     |
339  |   rustc_query_append! { [define_callbacks!][<'tcx>] }
     |   --------------------------------------------------- in this macro invocation
     |   --------------------------------------------------- in this macro invocation
     |
     = note: consider importing one of these items:
             crate::ty::FxHashSet
             rustc_data_structures::stable_set::FxHashSet

   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: the trait bound `rustc_data_structures::stable_set::StableSet<rustc_span::Symbol>: ArenaAllocatable<'_, _>` is not satisfied
    --> compiler/rustc_middle/src/ty/context.rs:2988:25
     |
2988 |         tcx.arena.alloc(tcx.resolutions(()).glob_map.get(&id).cloned().unwrap_or_default())
     |                   ----- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an implementor of trait `ArenaAllocatable<'_, _>`
     |                   required by a bound introduced by this call
     |
     |
     = note: required because of the requirements on the impl of `ArenaAllocatable<'_, IsCopy>` for `rustc_data_structures::stable_set::StableSet<rustc_span::Symbol>`
note: required by a bound in `Arena::<'tcx>::alloc`
    --> /checkout/compiler/rustc_arena/src/lib.rs:598:25
     |
542  | /  pub macro declare_arena([$($a:tt $name:ident: $ty:ty,)*]) {
544  | |      pub struct Arena<'tcx> {
544  | |      pub struct Arena<'tcx> {
545  | |          pub dropless: $crate::DroplessArena,
...    |
598  | |          pub fn alloc<T: ArenaAllocatable<'tcx, C>, C>(&self, value: T) -> &mut T {
     | |                          ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Arena::<'tcx>::alloc`
617  | |      }
618  | |  }
618  | |  }
     | |__- in this expansion of `rustc_arena::declare_arena!` (#2)
     |
    ::: compiler/rustc_middle/src/arena.rs:6:1
     |
6    |  / macro_rules! arena_types {
7    |  |     ($macro:path) => (
8    |  |         $macro!([
9    | ||             [] layout: rustc_target::abi::Layout,
9    | ||             [] layout: rustc_target::abi::Layout,
10   | ||             [] fn_abi: rustc_target::abi::call::FnAbi<'tcx, rustc_middle::ty::Ty<'tcx>>,
11   | ||             // AdtDef are interned and compared by address
...    ||
104  | ||             [] dep_kind: rustc_middle::dep_graph::DepKindStruct,
     | ||__________- in this macro invocation (#2)
106  |  |     )
107  |  | }
107  |  | }
     |  |_- in this expansion of `arena_types!` (#1)
108  | 
109  |    arena_types!(rustc_arena::declare_arena);
help: consider borrowing here
     |
     |
2988 |         tcx.arena.alloc(&tcx.resolutions(()).glob_map.get(&id).cloned().unwrap_or_default())

error[E0283]: type annotations needed
    --> compiler/rustc_middle/src/ty/query.rs:216:28
     |
     |
178  | /   macro_rules! define_callbacks {
179  | |       (<$tcx:tt>
180  | |        $($(#[$attr:meta])*
181  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
214  | |           #[derive(Default)]
     | |                    ------- in this derive macro expansion (#3)
     | |                    ------- in this derive macro expansion (#3)
215  | |           pub struct QueryCaches<$tcx> {
216  | |               $($(#[$attr])* pub $name: QueryCacheStore<query_storage::$name<$tcx>>,)*
...    |
324  | |       };
325  | |   }
325  | |   }
     | |___- in this expansion of `define_callbacks!` (#2)
339  |     rustc_query_append! { [define_callbacks!][<'tcx>] }
     |     --------------------------------------------------- in this macro invocation (#1)
     |
    ::: compiler/rustc_middle/src/query/mod.rs:18:1
