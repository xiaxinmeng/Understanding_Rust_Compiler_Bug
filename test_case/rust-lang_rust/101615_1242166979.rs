plain
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0277]: the trait bound `for<'a> &HashMap<DefId, rustc_middle::ty::Ty<'_>, BuildHasherDefault<FxHasher>>: Decodable<CacheDecoder<'a, '_>>` is not satisfied
   ::: compiler/rustc_query_impl/src/plumbing.rs:389:13
    |
389 |               rustc_query_description! { $name }
    |               ---------------------------------- in this macro invocation (#2)
    |               ---------------------------------- in this macro invocation (#2)
    |
   ::: compiler/rustc_query_impl/src/lib.rs:54:1
    |
54  |   rustc_query_append! { define_queries! }
    |
    |
    = help: the trait `Decodable<D>` is implemented for `HashMap<K, V, S>`
    = note: required because of the requirements on the impl of `for<'a> Decodable<CacheDecoder<'a, '_>>` for `Result<&HashMap<DefId, rustc_middle::ty::Ty<'_>, BuildHasherDefault<FxHasher>>, ErrorGuaranteed>`
note: required by a bound in `on_disk_cache::OnDiskCache::<'sess>::try_load_query_result`
   --> compiler/rustc_query_impl/src/on_disk_cache.rs:399:12
393 |     pub fn try_load_query_result<'tcx, T>(
    |            --------------------- required by a bound in this
...
...
399 |         T: for<'a> Decodable<CacheDecoder<'a, 'tcx>>,
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `on_disk_cache::OnDiskCache::<'sess>::try_load_query_result`
For more information about this error, try `rustc --explain E0277`.
[RUSTC-TIMING] rustc_query_impl test:false 3.085
error: could not compile `rustc_query_impl` due to previous error
warning: build failed, waiting for other jobs to finish...
