plain
    |
146 | {
    | - unclosed delimiter
...
150 |     fn lookup(&self, _key: &()) -> Option<(V, DepNodeIndex)> {
    |                                                              - this delimiter might not be properly closed...
168 | }
168 | }
    | - ...as it matches this but it has different indentation
391 | }
    |   ^


error: struct is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:170:1
    |
170 | pub struct ArenaCache<'tcx, K, V> {
    |
    |
    = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:178:1
    |
178 | impl<'tcx, K, V> Default for ArenaCache<'tcx, K, V> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:184:1
    |
184 | impl<'tcx, K: Eq + Hash, V: Debug + 'tcx> QueryStorage for ArenaCache<'tcx, K, V> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:189:1
    |
189 | / impl<'tcx, K, V: 'tcx> QueryCache for ArenaCache<'tcx, K, V>
190 | | where
191 | |     K: Eq + Hash + Clone + Debug,
192 | |     V: Debug,
239 | |     }
240 | | }
    | |_^
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:242:1
    |
242 | pub struct VecCacheSelector<K>(PhantomData<K>);
    |
    |
    = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:244:1
    |
244 | impl<'tcx, K: Idx, V: 'tcx> CacheSelector<'tcx, V> for VecCacheSelector<K> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:251:1
    |
251 | pub struct VecCache<K: Idx, V> {
    |
    |
    = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:258:1
    |
258 | impl<K: Idx, V> Default for VecCache<K, V> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:264:1
    |
264 | impl<K: Eq + Idx, V: Copy + Debug> QueryStorage for VecCache<K, V> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:269:1
    |
269 | / impl<K, V> QueryCache for VecCache<K, V>
270 | | where
271 | |     K: Eq + Idx + Clone + Debug,
272 | |     V: Copy + Debug,
316 | |     }
317 | | }
    | |_^
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: struct is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:319:1
    |
319 | pub struct VecArenaCache<'tcx, K: Idx, V> {
    |
    |
    = help: consider moving the struct out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:327:1
    |
327 | impl<'tcx, K: Idx, V> Default for VecArenaCache<'tcx, K, V> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:336:1
    |
336 | impl<'tcx, K: Eq + Idx, V: Debug + 'tcx> QueryStorage for VecArenaCache<'tcx, K, V> {
    |
    |
    = help: consider moving the implementation out to a nearby module scope

error: implementation is not supported in `trait`s or `impl`s
   --> compiler/rustc_query_system/src/query/caches.rs:341:1
    |
341 | / impl<'tcx, K, V: 'tcx> QueryCache for VecArenaCache<'tcx, K, V>
342 | | where
343 | |     K: Eq + Idx + Clone + Debug,
344 | |     V: Debug,
390 | |     }
391 | | }
    | |_^
    |
    |
    = help: consider moving the implementation out to a nearby module scope
error: `self` parameter is only allowed in associated functions
   --> compiler/rustc_query_system/src/query/caches.rs:160:17
    |
    |
160 |     fn complete(&self, _key: (), value: V, index: DepNodeIndex) -> Self::Stored {
    |                 ^^^^^ not semantically valid as function parameter
    |
    = note: associated functions are those in `impl` or `trait` definitions
error: `self` parameter is only allowed in associated functions
   --> compiler/rustc_query_system/src/query/caches.rs:165:13
    |
    |
165 |     fn iter(&self, f: &mut dyn FnMut(&Self::Key, &Self::Value, DepNodeIndex)) {
    |             ^^^^^ not semantically valid as function parameter
    |
    = note: associated functions are those in `impl` or `trait` definitions
error[E0432]: unresolved import `self::caches::VecCacheSelector`
  --> compiler/rustc_query_system/src/query/mod.rs:12:5
   |
12 |     VecCacheSelector,
12 |     VecCacheSelector,
   |     ^^^^^^^^^^^^^^^^
   |     |
   |     no `VecCacheSelector` in `query::caches`
   |     help: a similar name exists in the module: `CacheSelector`
error[E0401]: can't use generic parameters from outer function
   --> compiler/rustc_query_system/src/query/caches.rs:160:17
    |
    |
143 | impl<V> QueryCache for SingleCache<V>
    | ---- `Self` type implicitly declared here, by this `impl`
...
160 |     fn complete(&self, _key: (), value: V, index: DepNodeIndex) -> Self::Stored {
    |                 |
    |                 use of generic parameter from outer function
    |                 use a type here instead


error[E0401]: can't use generic parameters from outer function
   --> compiler/rustc_query_system/src/query/caches.rs:160:41
    |
143 | impl<V> QueryCache for SingleCache<V>
    |      - type parameter from outer function
...
160 |     fn complete(&self, _key: (), value: V, index: DepNodeIndex) -> Self::Stored {
    |                -                        ^ use of generic parameter from outer function
    |                |
    |                help: try using a local generic parameter instead: `<V>`
error[E0401]: can't use generic parameters from outer function
   --> compiler/rustc_query_system/src/query/caches.rs:160:68
    |
    |
143 | impl<V> QueryCache for SingleCache<V>
    | ---- `Self` type implicitly declared here, by this `impl`
...
160 |     fn complete(&self, _key: (), value: V, index: DepNodeIndex) -> Self::Stored {
    |                                                                    |
    |                                                                    use of generic parameter from outer function
    |                                                                    use a type here instead


error[E0401]: can't use generic parameters from outer function
   --> compiler/rustc_query_system/src/query/caches.rs:165:13
    |
143 | impl<V> QueryCache for SingleCache<V>
    | ---- `Self` type implicitly declared here, by this `impl`
...
165 |     fn iter(&self, f: &mut dyn FnMut(&Self::Key, &Self::Value, DepNodeIndex)) {
    |             |
    |             use of generic parameter from outer function
    |             use a type here instead


error[E0401]: can't use generic parameters from outer function
   --> compiler/rustc_query_system/src/query/caches.rs:165:39
    |
143 | impl<V> QueryCache for SingleCache<V>
    | ---- `Self` type implicitly declared here, by this `impl`
...
165 |     fn iter(&self, f: &mut dyn FnMut(&Self::Key, &Self::Value, DepNodeIndex)) {
    |                                       |
    |                                       use of generic parameter from outer function
    |                                       use a type here instead


error[E0401]: can't use generic parameters from outer function
   --> compiler/rustc_query_system/src/query/caches.rs:165:51
    |
143 | impl<V> QueryCache for SingleCache<V>
    | ---- `Self` type implicitly declared here, by this `impl`
...
165 |     fn iter(&self, f: &mut dyn FnMut(&Self::Key, &Self::Value, DepNodeIndex)) {
    |                                                   |
    |                                                   use of generic parameter from outer function
    |                                                   use a type here instead


error[E0412]: cannot find type `ArenaCache` in this scope
  --> compiler/rustc_query_system/src/query/caches.rs:47:23
   |
47 |     type ArenaCache = ArenaCache<'tcx, K, V>;
   |                       |
   |                       |
   |                       help: you might have meant to use the associated type: `Self::ArenaCache`

error[E0412]: cannot find type `ArenaCache` in this scope
   --> compiler/rustc_query_system/src/query/caches.rs:125:23
    |
125 |     type ArenaCache = ArenaCache<'tcx, (), V>;
    |                       |
    |                       |
    |                       help: you might have meant to use the associated type: `Self::ArenaCache`

error: unused import: `rustc_arena::TypedArena`
 --> compiler/rustc_query_system/src/query/caches.rs:3:5
  |
3 | use rustc_arena::TypedArena;
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_data_structures::sync::WorkerLocal`
 --> compiler/rustc_query_system/src/query/caches.rs:9:5
  |
9 | use rustc_data_structures::sync::WorkerLocal;
9 | use rustc_data_structures::sync::WorkerLocal;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: unused import: `IndexVec`
  --> compiler/rustc_query_system/src/query/caches.rs:10:29
   |
10 | use rustc_index::vec::{Idx, IndexVec};


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

error[E0046]: not all trait items implemented, missing: `complete`, `iter`
   --> compiler/rustc_query_system/src/query/caches.rs:143:1
    |
    |
36  |     fn complete(&self, key: Self::Key, value: Self::Value, index: DepNodeIndex) -> Self::Stored;
    |     -------------------------------------------------------------------------------------------- `complete` from trait
37  |
38  |     fn iter(&self, f: &mut dyn FnMut(&Self::Key, &Self::Value, DepNodeIndex));
    |     -------------------------------------------------------------------------- `iter` from trait
...
143 | impl<V> QueryCache for SingleCache<V>
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `complete`, `iter` in implementation
error[E0308]: mismatched types
   --> compiler/rustc_query_system/src/query/caches.rs:153:9
    |
    |
153 | /         match *cache {
154 | |             Some(ref value) => Ok((value.0, value.1)),
155 | |             None => Err(()),
156 | |         }
    | |         ^- help: consider using a semicolon here: `;`
    |           expected `()`, found enum `Result`
    |
    = note: expected unit type `()`
    = note: expected unit type `()`
                    found enum `Result<(V, DepNodeIndex), ()>`
error[E0308]: mismatched types
   --> compiler/rustc_query_system/src/query/caches.rs:150:36
    |
    |
150 |     fn lookup(&self, _key: &()) -> Option<(V, DepNodeIndex)> {
    |        |
    |        |
    |        implicitly returns `()` as its body has no tail or `return` expression
    |
    = note:   expected enum `Option<(V, DepNodeIndex)>`

error: unused import: `Idx`
  --> compiler/rustc_query_system/src/query/caches.rs:10:24
   |
   |
10 | use rustc_index::vec::{Idx, IndexVec};

Some errors have detailed explanations: E0046, E0277, E0308, E0401, E0412, E0432.
For more information about an error, try `rustc --explain E0046`.
error: could not compile `rustc_query_system` due to 35 previous errors
