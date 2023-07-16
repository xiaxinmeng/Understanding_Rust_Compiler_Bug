plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0412]: cannot find type `FxHashSet` in this scope
   --> compiler/rustc_query_impl/src/on_disk_cache.rs:755:60
    |
755 | impl<'a, 'tcx> Decodable<CacheDecoder<'a, 'tcx>> for &'tcx FxHashSet<LocalDefId> {
    |
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-hash-1.1.0/src/lib.rs:43:1
    |
    |
43  | pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
    | ----------------------------------------------------------------------- similarly named type alias `FxHashMap` defined here
help: a type alias with a similar name exists
    |
    |
755 | impl<'a, 'tcx> Decodable<CacheDecoder<'a, 'tcx>> for &'tcx FxHashMap<LocalDefId> {
help: consider importing this type alias
    |
    |
1   | use rustc_data_structures::fx::FxHashSet;

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0277]: the trait bound `for<'a> &HashSet<LocalDefId, BuildHasherDefault<FxHasher>>: Decodable<CacheDecoder<'a, '_>>` is not satisfied
    ::: compiler/rustc_query_impl/src/lib.rs:56:1
     |
56   |   rustc_query_append! { [define_queries!][<'tcx>] }
     |   ------------------------------------------------- in this macro invocation (#1)
     |   ------------------------------------------------- in this macro invocation (#1)
     |
    ::: compiler/rustc_query_impl/src/plumbing.rs:323:13
     |
323  |               rustc_query_description! { $name<$tcx> }
    --> /checkout/compiler/rustc_middle/src/query/mod.rs:18:1
     |
18   | / rustc_queries! {
19   | |     query trigger_delay_span_bug(key: DefId) -> () {
19   | |     query trigger_delay_span_bug(key: DefId) -> () {
20   | |         desc { "trigger a delay span bug" }
21   | |     }
...    |
2055 | |     }
2056 | | }
     | | ^
     | | |
     | | the trait `for<'a> Decodable<CacheDecoder<'a, '_>>` is not implemented for `&HashSet<LocalDefId, BuildHasherDefault<FxHasher>>`
     | |_in this expansion of `rustc_query_append!` (#1)
     |   in this expansion of `rustc_query_description!` (#2)
     |
     = help: the trait `Decodable<D>` is implemented for `HashSet<T, S>`
note: required by a bound in `on_disk_cache::OnDiskCache::<'sess>::try_load_query_result`
    --> compiler/rustc_query_impl/src/on_disk_cache.rs:393:12
387  |     pub fn try_load_query_result<'tcx, T>(
     |            --------------------- required by a bound in this
...
...
393  |         T: for<'a> Decodable<CacheDecoder<'a, 'tcx>>,
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `on_disk_cache::OnDiskCache::<'sess>::try_load_query_result`
Some errors have detailed explanations: E0277, E0412.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_query_impl` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
