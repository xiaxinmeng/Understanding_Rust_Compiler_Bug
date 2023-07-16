plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0599]: no method named `extend` found for struct `StableSet` in the current scope
   --> compiler/rustc_passes/src/liveness.rs:283:41
    |
283 |                     shorthand_field_ids.extend(short.iter().map(|f| f.pat.hir_id));
    |                                         ^^^^^^ method not found in `StableSet<HirId>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error[E0277]: the trait bound `for<'a> HashSet<LocalDefId, BuildHasherDefault<FxHasher>>: rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` is not satisfied
    --> compiler/rustc_query_impl/src/plumbing.rs:218:14
216  | /    macro_rules! hash_result {
217  | |        ([]) => {{
218  | |            Some(dep_graph::hash_result)
218  | |            Some(dep_graph::hash_result)
     | |                 ^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` is not implemented for `HashSet<LocalDefId, BuildHasherDefault<FxHasher>>`
219  | |        }};
...    |
224  | |            hash_result!([$($modifiers)*])
225  | |        };
226  | |    }
     | |    -
     | |    |
     | |    |
     | |____in this expansion of `hash_result!` (#3)
     |      in this expansion of `hash_result!` (#4)
254  | /    macro_rules! define_queries {
254  | /    macro_rules! define_queries {
255  | |        (<$tcx:tt>
256  | |         $($(#[$attr:meta])*
257  | |            [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
355  | |                        hash_result: hash_result!([$($modifiers)*]),
...    |
468  | |        }
469  | |    }
469  | |    }
     | |____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:55:1
     |
55   |      rustc_query_append! { [define_queries!][<'tcx>] }
     |      ------------------------------------------------- in this macro invocation (#1)
---
     |    | |
     |    |_in this expansion of `rustc_query_append!` (#1)
     |      in this macro invocation (#2)
     |
     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` for `(HashSet<LocalDefId, BuildHasherDefault<FxHasher>>, HashMap<LocalDefId, Vec<(DefId, DefId)>, BuildHasherDefault<FxHasher>>)`
note: required by a bound in `hash_result`
    --> /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:101:8
     |
101  |     R: for<'a> HashStable<StableHashingContext<'a>>,
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `hash_result`

error[E0277]: the trait bound `for<'a> HashSet<LocalDefId, BuildHasherDefault<FxHasher>>: rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` is not satisfied
    --> compiler/rustc_query_impl/src/plumbing.rs:218:14
216  | /   macro_rules! hash_result {
217  | |       ([]) => {{
218  | |           Some(dep_graph::hash_result)
218  | |           Some(dep_graph::hash_result)
     | |                ^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` is not implemented for `HashSet<LocalDefId, BuildHasherDefault<FxHasher>>`
219  | |       }};
225  | |       };
226  | |   }
226  | |   }
     | |___- in this expansion of `hash_result!` (#3)
...
254  | /   macro_rules! define_queries {
255  | |       (<$tcx:tt>
256  | |        $($(#[$attr:meta])*
257  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
355  | |                       hash_result: hash_result!([$($modifiers)*]),
...    |
468  | |       }
469  | |   }
469  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:55:1
     |
55   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)
     |
     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` for `&HashSet<LocalDefId, BuildHasherDefault<FxHasher>>`
note: required by a bound in `hash_result`
    --> /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:101:8
     |
101  |     R: for<'a> HashStable<StableHashingContext<'a>>,
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `hash_result`

error[E0277]: the trait bound `for<'a> HashSet<LocalDefId, BuildHasherDefault<FxHasher>>: rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` is not satisfied
    --> compiler/rustc_query_impl/src/plumbing.rs:218:14
216  | /    macro_rules! hash_result {
217  | |        ([]) => {{
218  | |            Some(dep_graph::hash_result)
218  | |            Some(dep_graph::hash_result)
     | |                 ^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` is not implemented for `HashSet<LocalDefId, BuildHasherDefault<FxHasher>>`
219  | |        }};
...    |
224  | |            hash_result!([$($modifiers)*])
225  | |        };
226  | |    }
     | |    -
     | |    |
     | |    |
     | |____in this expansion of `hash_result!` (#3)
     |      in this expansion of `hash_result!` (#4)
254  | /    macro_rules! define_queries {
254  | /    macro_rules! define_queries {
255  | |        (<$tcx:tt>
256  | |         $($(#[$attr:meta])*
257  | |            [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
355  | |                        hash_result: hash_result!([$($modifiers)*]),
...    |
468  | |        }
469  | |    }
469  | |    }
     | |____- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:55:1
     |
55   |      rustc_query_append! { [define_queries!][<'tcx>] }
     |      ------------------------------------------------- in this macro invocation (#1)
---
     |
note: required by a bound in `hash_result`
    --> /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:101:8
     |
101  |     R: for<'a> HashStable<StableHashingContext<'a>>,
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `hash_result`

error[E0277]: the trait bound `for<'a> HashSet<ItemLocalId, BuildHasherDefault<FxHasher>>: rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` is not satisfied
    --> compiler/rustc_query_impl/src/plumbing.rs:218:14
216  | /   macro_rules! hash_result {
217  | |       ([]) => {{
218  | |           Some(dep_graph::hash_result)
218  | |           Some(dep_graph::hash_result)
     | |                ^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` is not implemented for `HashSet<ItemLocalId, BuildHasherDefault<FxHasher>>`
219  | |       }};
225  | |       };
226  | |   }
226  | |   }
     | |___- in this expansion of `hash_result!` (#3)
...
254  | /   macro_rules! define_queries {
255  | |       (<$tcx:tt>
256  | |        $($(#[$attr:meta])*
257  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
355  | |                       hash_result: hash_result!([$($modifiers)*]),
...    |
468  | |       }
469  | |   }
469  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:55:1
     |
55   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)
     |
     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` for `&HashSet<ItemLocalId, BuildHasherDefault<FxHasher>>`
     = note: 2 redundant requirements hidden
     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` for `Option<(LocalDefId, &HashSet<ItemLocalId, BuildHasherDefault<FxHasher>>)>`
note: required by a bound in `hash_result`
    --> /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:101:8
     |
101  |     R: for<'a> HashStable<StableHashingContext<'a>>,
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `hash_result`

error[E0277]: the trait bound `for<'a> HashSet<Symbol, BuildHasherDefault<FxHasher>>: rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` is not satisfied
    --> compiler/rustc_query_impl/src/plumbing.rs:218:14
216  | /   macro_rules! hash_result {
217  | |       ([]) => {{
218  | |           Some(dep_graph::hash_result)
218  | |           Some(dep_graph::hash_result)
     | |                ^^^^^^^^^^^^^^^^^^^^^^ the trait `for<'a> rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` is not implemented for `HashSet<Symbol, BuildHasherDefault<FxHasher>>`
219  | |       }};
225  | |       };
226  | |   }
226  | |   }
     | |___- in this expansion of `hash_result!` (#3)
...
254  | /   macro_rules! define_queries {
255  | |       (<$tcx:tt>
256  | |        $($(#[$attr:meta])*
257  | |           [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
355  | |                       hash_result: hash_result!([$($modifiers)*]),
...    |
468  | |       }
469  | |   }
469  | |   }
     | |___- in this expansion of `define_queries!` (#2)
    ::: compiler/rustc_query_impl/src/lib.rs:55:1
     |
55   |     rustc_query_append! { [define_queries!][<'tcx>] }
     |     ------------------------------------------------- in this macro invocation (#1)
---
     |   | |
     |   |_in this expansion of `rustc_query_append!` (#1)
     |     in this macro invocation (#2)
     |
     = note: required because of the requirements on the impl of `for<'a> rustc_data_structures::stable_hasher::HashStable<StableHashingContext<'a>>` for `&HashSet<Symbol, BuildHasherDefault<FxHasher>>`
note: required by a bound in `hash_result`
    --> /checkout/compiler/rustc_query_system/src/dep_graph/graph.rs:101:8
     |
101  |     R: for<'a> HashStable<StableHashingContext<'a>>,
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `hash_result`
For more information about this error, try `rustc --explain E0277`.
error: build failed
Build completed unsuccessfully in 0:02:45
