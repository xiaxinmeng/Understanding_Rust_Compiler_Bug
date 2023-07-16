plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0277]: `Lrc<(dyn std::any::Any + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `Lrc<(dyn std::any::Any + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `Lrc<(dyn std::any::Any + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `Lrc<Atomic<u32>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `Lrc<Atomic<u32>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `Lrc<Atomic<u32>>`
    = note: required because it appears within the type `DepGraph<DepKind>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Ty<'_>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Ty<'_>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Ty<'_>, BuildHasherDefault<FxHasher>>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<HashMap<CReaderCacheKey, Ty<'_>, BuildHasherDefault<FxHasher>>>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Predicate<'_>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Predicate<'_>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Predicate<'_>, BuildHasherDefault<FxHasher>>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<HashMap<CReaderCacheKey, Predicate<'_>, BuildHasherDefault<FxHasher>>>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<rustc_middle::mir::interpret::AllocMap<'_>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<rustc_middle::mir::interpret::AllocMap<'_>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<rustc_middle::mir::interpret::AllocMap<'_>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<AllocMap<'_>>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `Lrc<SearchPath>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `Lrc<SearchPath>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `Lrc<SearchPath>`
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `OnceCell<Vec<CrateType>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `OnceCell<Vec<CrateType>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `OnceCell<Vec<CrateType>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::OnceLock` instead
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `OnceCell<StableCrateId>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `OnceCell<StableCrateId>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `OnceCell<StableCrateId>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::OnceLock` instead
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `OnceCell<rustc_feature::active::Features>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `OnceCell<rustc_feature::active::Features>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `OnceCell<rustc_feature::active::Features>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::OnceLock` instead
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `Lrc<DepGraphData<rustc_middle::dep_graph::DepKind>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `Lrc<DepGraphData<rustc_middle::dep_graph::DepKind>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `Lrc<DepGraphData<rustc_middle::dep_graph::DepKind>>`
    = note: required because it appears within the type `Option<Rc<DepGraphData<DepKind>>>`
    = note: required because it appears within the type `DepGraph<DepKind>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<std::boxed::Box<(dyn CrateStore + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<std::boxed::Box<(dyn CrateStore + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<std::boxed::Box<(dyn CrateStore + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `RwLock<Box<dyn CrateStore + Send + Sync>>`
    = note: required because it appears within the type `Untracked`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<Definitions>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<Definitions>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<Definitions>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `RwLock<Definitions>`
    = note: required because it appears within the type `Untracked`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `Cell<u64>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `Cell<u64>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `Cell<u64>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` or `std::sync::atomic::AtomicU64` instead
    = note: required because it appears within the type `Atomic<u64>`
    = note: required because it appears within the type `QuerySystem<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<std::result::Result<std::option::Option<SelectionCandidate<'_>>, SelectionError<'_>>>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<std::result::Result<std::option::Option<SelectionCandidate<'_>>, SelectionError<'_>>>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<std::result::Result<std::option::Option<SelectionCandidate<'_>>, SelectionError<'_>>>, BuildHasherDefault<FxHasher>>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<HashMap<(ParamEnv<'_>, TraitPredicate<'_>), WithDepNode<Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>, BuildHasherDefault<FxHasher>>>`
    = note: required because it appears within the type `Cache<(ParamEnv<'_>, TraitPredicate<'_>), Result<Option<SelectionCandidate<'_>>, SelectionError<'_>>>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<HashMap<(ParamEnv<'_>, rustc_middle::ty::Binder<'_, TraitPredicate<'_>>), WithDepNode<EvaluationResult>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<HashMap<(ParamEnv<'_>, rustc_middle::ty::Binder<'_, TraitPredicate<'_>>), WithDepNode<EvaluationResult>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<HashMap<(ParamEnv<'_>, rustc_middle::ty::Binder<'_, TraitPredicate<'_>>), WithDepNode<EvaluationResult>, BuildHasherDefault<FxHasher>>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<HashMap<(ParamEnv<'_>, Binder<'_, TraitPredicate<'_>>), WithDepNode<EvaluationResult>, BuildHasherDefault<FxHasher>>>`
    = note: required because it appears within the type `Cache<(ParamEnv<'_>, Binder<'_, TraitPredicate<'_>>), EvaluationResult>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<HashMap<rustc_middle::infer::canonical::Canonical<'_, Goal<'_, rustc_middle::ty::Predicate<'_>>>, WithDepNode<std::result::Result<rustc_middle::infer::canonical::Canonical<'_, Response<'_>>, NoSolution>>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<HashMap<rustc_middle::infer::canonical::Canonical<'_, Goal<'_, rustc_middle::ty::Predicate<'_>>>, WithDepNode<std::result::Result<rustc_middle::infer::canonical::Canonical<'_, Response<'_>>, NoSolution>>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<HashMap<rustc_middle::infer::canonical::Canonical<'_, Goal<'_, rustc_middle::ty::Predicate<'_>>>, WithDepNode<std::result::Result<rustc_middle::infer::canonical::Canonical<'_, Response<'_>>, NoSolution>>, BuildHasherDefault<FxHasher>>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<HashMap<Canonical<'_, Goal<'_, Predicate<'_>>>, WithDepNode<Result<Canonical<'_, Response<'_>>, NoSolution>>, BuildHasherDefault<FxHasher>>>`
    = note: required because it appears within the type `Cache<Canonical<'_, Goal<'_, Predicate<'_>>>, Result<Canonical<'_, Response<'_>>, NoSolution>>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `Lrc<SourceMap>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `Lrc<SourceMap>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `Lrc<SourceMap>`
    = note: required because it appears within the type `ParseSess`
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<IncrCompSession>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<IncrCompSession>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<IncrCompSession>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `OneThread<RefCell<IncrCompSession>>`
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<OptimizationFuel>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<OptimizationFuel>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<OptimizationFuel>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<OptimizationFuel>`
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<CtfeBacktrace>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<CtfeBacktrace>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<CtfeBacktrace>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<CtfeBacktrace>`
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `RefCell<Vec<(rustc_span::Span, std::option::Option<rustc_span::Symbol>)>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `RefCell<Vec<(rustc_span::Span, std::option::Option<rustc_span::Symbol>)>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `RefCell<Vec<(rustc_span::Span, std::option::Option<rustc_span::Symbol>)>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<Vec<(Span, Option<Symbol>)>>`
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`

error[E0277]: `std::cell::UnsafeCell<Vec<rustc_span::Span>>` cannot be shared between threads safely
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::cell::UnsafeCell<Vec<rustc_span::Span>>` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `std::cell::UnsafeCell<Vec<rustc_span::Span>>`
    = note: required because it appears within the type `FrozenVec<Span>`
    = note: required because it appears within the type `AppendOnlyIndexVec<LocalDefId, Span>`
    = note: required because it appears within the type `Untracked`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
    |
615 | pub fn print_query_stack<Qcx: QueryContext + Send>(
    |                                              ^^^^ required by this bound in `print_query_stack`
error[E0277]: `rustc_span::Span` cannot be shared between threads safely
   --> compiler/rustc_interface/src/interface.rs:341:31
    |
    |
341 |             print_query_stack(QueryCtxt { tcx: icx.tcx }, icx.query, handler, num_frames)
    |             ----------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^ `rustc_span::Span` cannot be shared between threads safely
    |             required by a bound introduced by this call
    |
    |
    = help: within `GlobalCtxt<'_>`, the trait `std::marker::Sync` is not implemented for `rustc_span::Span`
    = note: required because it appears within the type `PhantomData<Span>`
    = note: required because it appears within the type `QueryPhantomValues<'_>`
    = note: required because it appears within the type `QuerySystem<'_>`
    = note: required because it appears within the type `GlobalCtxt<'_>`
    = note: required for `&GlobalCtxt<'_>` to implement `std::marker::Send`
    = note: required because it appears within the type `QueryCtxt<'_>`
note: required by a bound in `print_query_stack`
   --> /checkout/compiler/rustc_query_system/src/query/job.rs:615:46
    |
