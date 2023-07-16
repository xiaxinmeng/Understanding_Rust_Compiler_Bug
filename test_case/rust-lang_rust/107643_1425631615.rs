plain
[RUSTC-TIMING] rustc_session test:false 0.980
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0407]: method `store_nocache` is not a member of trait `QueryStorage`
   --> compiler/rustc_query_system/src/query/caches.rs:143:5
    |
143 | /     fn store_nocache(&self, value: Self::Value) -> Self::Stored {
145 | |     }
145 | |     }
    | |_____^ not a member of trait `QueryStorage`

error[E0277]: the trait bound `V: std::marker::Copy` is not satisfied
   --> compiler/rustc_query_system/src/query/caches.rs:122:18
    |
122 |     type Cache = SingleCache<V>
    |                  ^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `V`
help: consider further restricting this bound
    |
    |
121 | impl<'tcx, V: 'tcx + std::marker::Copy> CacheSelector<'tcx, V> for SingleCacheSelector {


error[E0277]: the trait bound `V: std::marker::Copy` is not satisfied
   --> compiler/rustc_query_system/src/query/caches.rs:140:19
140 |     type Stored = V;
    |                   ^ the trait `std::marker::Copy` is not implemented for `V`
    |
note: required by a bound in `QueryStorage::Stored`
note: required by a bound in `QueryStorage::Stored`
   --> compiler/rustc_query_system/src/query/caches.rs:24:18
    |
24  |     type Stored: Copy;
    |                  ^^^^ required by this bound in `QueryStorage::Stored`
help: consider further restricting this bound
    |
138 | impl<V: Clone + Debug + std::marker::Copy> QueryStorage for SingleCache<V> {


error[E0049]: method `lookup` has 2 type parameters but its trait declaration has 0 type parameters
   --> compiler/rustc_query_system/src/query/caches.rs:155:15
    |
34  |     fn lookup(&self, key: &Self::Key) -> Option<(Self::Stored, DepNodeIndex)>;
    |              - expected 0 type parameters
...
155 |     fn lookup<R, OnHit>(&self, _key: &(), on_hit: OnHit) -> Result<R, ()>
    |               |
    |               found 2 type parameters

[RUSTC-TIMING] rustc_attr test:true 0.279
