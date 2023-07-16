plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error[E0107]: missing generics for struct `CycleError`
   --> compiler/rustc_query_system/src/query/job.rs:191:24
    |
191 |     cycle: Lock<Option<CycleError>>,
    |                        ^^^^^^^^^^ expected 1 generic argument
    |
note: struct defined here, with 1 generic parameter: `D`
   --> compiler/rustc_query_system/src/query/plumbing.rs:311:19
    |
311 | pub(crate) struct CycleError<D: DepKind> {
help: add missing generic argument
    |
    |
191 |     cycle: Lock<Option<CycleError<D>>>,


error[E0107]: missing generics for type alias `QueryMap`
   --> compiler/rustc_query_system/src/query/job.rs:520:28
    |
520 | pub fn deadlock(query_map: QueryMap, registry: &rayon_core::Registry) {
    |                            ^^^^^^^^ expected 1 generic argument
    |
note: type alias defined here, with 1 generic parameter: `D`
   --> compiler/rustc_query_system/src/query/job.rs:38:10
    |
38  | pub type QueryMap<D> = FxHashMap<QueryJobId, QueryJobInfo<D>>;
help: add missing generic argument
    |
    |
520 | pub fn deadlock(query_map: QueryMap<D>, registry: &rayon_core::Registry) {


error[E0107]: missing generics for type alias `QueryMap`
   --> compiler/rustc_query_system/src/query/job.rs:294:33
    |
294 | fn visit_waiters<F>(query_map: &QueryMap, query: QueryJobId, mut visit: F) -> Option<Option<Waiter>>
    |                                 ^^^^^^^^ expected 1 generic argument
    |
note: type alias defined here, with 1 generic parameter: `D`
   --> compiler/rustc_query_system/src/query/job.rs:38:10
    |
38  | pub type QueryMap<D> = FxHashMap<QueryJobId, QueryJobInfo<D>>;
help: add missing generic argument
    |
    |
294 | fn visit_waiters<F>(query_map: &QueryMap<D>, query: QueryJobId, mut visit: F) -> Option<Option<Waiter>>


error[E0107]: missing generics for type alias `QueryMap`
   --> compiler/rustc_query_system/src/query/job.rs:326:17
    |
326 |     query_map: &QueryMap,
    |                 ^^^^^^^^ expected 1 generic argument
    |
note: type alias defined here, with 1 generic parameter: `D`
   --> compiler/rustc_query_system/src/query/job.rs:38:10
    |
38  | pub type QueryMap<D> = FxHashMap<QueryJobId, QueryJobInfo<D>>;
help: add missing generic argument
    |
    |
326 |     query_map: &QueryMap<D>,


error[E0107]: missing generics for type alias `QueryMap`
   --> compiler/rustc_query_system/src/query/job.rs:367:17
    |
367 |     query_map: &QueryMap,
    |                 ^^^^^^^^ expected 1 generic argument
    |
note: type alias defined here, with 1 generic parameter: `D`
   --> compiler/rustc_query_system/src/query/job.rs:38:10
    |
38  | pub type QueryMap<D> = FxHashMap<QueryJobId, QueryJobInfo<D>>;
help: add missing generic argument
    |
    |
367 |     query_map: &QueryMap<D>,


error[E0107]: missing generics for type alias `QueryMap`
   --> compiler/rustc_query_system/src/query/job.rs:389:37
    |
389 | fn pick_query<'a, T, F>(query_map: &QueryMap, queries: &'a [T], f: F) -> &'a T
    |                                     ^^^^^^^^ expected 1 generic argument
    |
note: type alias defined here, with 1 generic parameter: `D`
   --> compiler/rustc_query_system/src/query/job.rs:38:10
    |
38  | pub type QueryMap<D> = FxHashMap<QueryJobId, QueryJobInfo<D>>;
help: add missing generic argument
    |
    |
389 | fn pick_query<'a, T, F>(query_map: &QueryMap<D>, queries: &'a [T], f: F) -> &'a T


error[E0107]: missing generics for type alias `QueryMap`
   --> compiler/rustc_query_system/src/query/job.rs:416:17
    |
416 |     query_map: &QueryMap,
    |                 ^^^^^^^^ expected 1 generic argument
    |
note: type alias defined here, with 1 generic parameter: `D`
   --> compiler/rustc_query_system/src/query/job.rs:38:10
    |
38  | pub type QueryMap<D> = FxHashMap<QueryJobId, QueryJobInfo<D>>;
help: add missing generic argument
    |
    |
416 |     query_map: &QueryMap<D>,


error[E0107]: missing generics for type alias `QueryMap`
  --> compiler/rustc_query_system/src/query/job.rs:50:25
   |
50 |     fn span(self, map: &QueryMap) -> Span {
   |                         ^^^^^^^^ expected 1 generic argument
   |
note: type alias defined here, with 1 generic parameter: `D`
  --> compiler/rustc_query_system/src/query/job.rs:38:10
   |
38 | pub type QueryMap<D> = FxHashMap<QueryJobId, QueryJobInfo<D>>;
help: add missing generic argument
   |
   |
50 |     fn span(self, map: &QueryMap<D>) -> Span {


error[E0107]: missing generics for type alias `QueryMap`
  --> compiler/rustc_query_system/src/query/job.rs:55:27
   |
55 |     fn parent(self, map: &QueryMap) -> Option<QueryJobId> {
   |                           ^^^^^^^^ expected 1 generic argument
   |
note: type alias defined here, with 1 generic parameter: `D`
  --> compiler/rustc_query_system/src/query/job.rs:38:10
   |
38 | pub type QueryMap<D> = FxHashMap<QueryJobId, QueryJobInfo<D>>;
help: add missing generic argument
   |
   |
55 |     fn parent(self, map: &QueryMap<D>) -> Option<QueryJobId> {


error[E0107]: missing generics for type alias `QueryMap`
  --> compiler/rustc_query_system/src/query/job.rs:60:33
   |
60 |     fn latch<'a>(self, map: &'a QueryMap) -> Option<&'a QueryLatch> {
   |                                 ^^^^^^^^ expected 1 generic argument
   |
note: type alias defined here, with 1 generic parameter: `D`
  --> compiler/rustc_query_system/src/query/job.rs:38:10
   |
38 | pub type QueryMap<D> = FxHashMap<QueryJobId, QueryJobInfo<D>>;
help: add missing generic argument
   |
   |
60 |     fn latch<'a>(self, map: &'a QueryMap<D>) -> Option<&'a QueryLatch> {


error[E0107]: missing generics for struct `CycleError`
   --> compiler/rustc_query_system/src/query/job.rs:223:87
    |
223 |     pub(super) fn wait_on(&self, query: Option<QueryJobId>, span: Span) -> Result<(), CycleError> {
    |                                                                                       ^^^^^^^^^^ expected 1 generic argument
    |
note: struct defined here, with 1 generic parameter: `D`
   --> compiler/rustc_query_system/src/query/plumbing.rs:311:19
    |
311 | pub(crate) struct CycleError<D: DepKind> {
help: add missing generic argument
    |
    |
223 |     pub(super) fn wait_on(&self, query: Option<QueryJobId>, span: Span) -> Result<(), CycleError<D>> {

For more information about this error, try `rustc --explain E0107`.
error: could not compile `rustc_query_system` due to 11 previous errors
warning: build failed, waiting for other jobs to finish...
