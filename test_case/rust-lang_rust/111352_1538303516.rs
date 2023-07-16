plain
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0277]: `Lrc<(dyn std::any::Any + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `Lrc<(dyn std::any::Any + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `Lrc<(dyn std::any::Any + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `RefCell<ThinVec<Diagnostic>>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `RefCell<ThinVec<Diagnostic>>` cannot be shared between threads safely
    |
    = help: within `ImplicitCtxt<'_, '_>`, the trait `std::marker::Sync` is not implemented for `RefCell<ThinVec<Diagnostic>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<ThinVec<Diagnostic>>`
    = note: required because it appears within the type `&Lock<ThinVec<Diagnostic>>`
    = note: required because it appears within the type `Option<&Lock<ThinVec<Diagnostic>>>`
    = note: required for `&ImplicitCtxt<'_, '_>` to implement `std::marker::Send`
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `RefCell<TaskDeps<rustc_middle::dep_graph::DepKind>>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `RefCell<TaskDeps<rustc_middle::dep_graph::DepKind>>` cannot be shared between threads safely
    |
    = help: within `ImplicitCtxt<'_, '_>`, the trait `std::marker::Sync` is not implemented for `RefCell<TaskDeps<rustc_middle::dep_graph::DepKind>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<TaskDeps<DepKind>>`
    = note: required because it appears within the type `&Lock<TaskDeps<DepKind>>`
    = note: required because it appears within the type `TaskDepsRef<'_, DepKind>`
    = note: required for `&ImplicitCtxt<'_, '_>` to implement `std::marker::Send`
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `Lrc<Atomic<u32>>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `Lrc<Atomic<u32>>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `Lrc<Atomic<u32>>`
    = note: required because it appears within the type `DepGraph<DepKind>`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Ty<'tcx>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Ty<'tcx>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Ty<'tcx>, BuildHasherDefault<FxHasher>>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<HashMap<CReaderCacheKey, Ty<'tcx>, BuildHasherDefault<FxHasher>>>`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Predicate<'tcx>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Predicate<'tcx>, BuildHasherDefault<FxHasher>>>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `RefCell<HashMap<CReaderCacheKey, rustc_middle::ty::Predicate<'tcx>, BuildHasherDefault<FxHasher>>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<HashMap<CReaderCacheKey, Predicate<'tcx>, BuildHasherDefault<FxHasher>>>`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `RefCell<interpret::AllocMap<'tcx>>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `RefCell<interpret::AllocMap<'tcx>>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `RefCell<interpret::AllocMap<'tcx>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `Lock<AllocMap<'tcx>>`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `Lrc<SearchPath>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `Lrc<SearchPath>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `Lrc<SearchPath>`
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `OnceCell<Vec<CrateType>>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `OnceCell<Vec<CrateType>>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `OnceCell<Vec<CrateType>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::OnceLock` instead
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `OnceCell<StableCrateId>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `OnceCell<StableCrateId>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `OnceCell<StableCrateId>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::OnceLock` instead
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `OnceCell<rustc_feature::active::Features>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `OnceCell<rustc_feature::active::Features>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `OnceCell<rustc_feature::active::Features>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::OnceLock` instead
    = note: required because it appears within the type `Session`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `Lrc<DepGraphData<rustc_middle::dep_graph::DepKind>>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `Lrc<DepGraphData<rustc_middle::dep_graph::DepKind>>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `Lrc<DepGraphData<rustc_middle::dep_graph::DepKind>>`
    = note: required because it appears within the type `Option<Rc<DepGraphData<DepKind>>>`
    = note: required because it appears within the type `DepGraph<DepKind>`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `RefCell<std::boxed::Box<(dyn CrateStore + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `RefCell<std::boxed::Box<(dyn CrateStore + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `RefCell<std::boxed::Box<(dyn CrateStore + rustc_data_structures::sync::Send + rustc_data_structures::sync::Sync + 'static)>>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `RwLock<Box<dyn CrateStore + Send + Sync>>`
    = note: required because it appears within the type `Untracked`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
    |                            ^^^^ required by this bound in `Scope::<'scope, 'env>::spawn`

error[E0277]: `RefCell<Definitions>` cannot be shared between threads safely
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                   s.spawn(move || {
    |  ___________________-----_^
    | |                   required by a bound introduced by this call
814 | |                     enter_context(context, || {
814 | |                     enter_context(context, || {
815 | |                         let query_map = qcx.try_collect_active_jobs();
816 | |                         tx.send(query_map).unwrap();
818 | |                 });
818 | |                 });
    | |_________________^ `RefCell<Definitions>` cannot be shared between threads safely
    |
    = help: within `GlobalCtxt<'tcx>`, the trait `std::marker::Sync` is not implemented for `RefCell<Definitions>`
    = note: if you want to do aliasing and mutation between multiple threads, use `std::sync::RwLock` instead
    = note: required because it appears within the type `RwLock<Definitions>`
    = note: required because it appears within the type `Untracked`
    = note: required because it appears within the type `GlobalCtxt<'tcx>`
    = note: required for `&'tcx GlobalCtxt<'tcx>` to implement `std::marker::Send`
note: required because it appears within the type `QueryCtxt<'tcx>`
   --> compiler/rustc_query_impl/src/plumbing.rs:37:12
    |
37  | pub struct QueryCtxt<'tcx> {
37  | pub struct QueryCtxt<'tcx> {
    |            ^^^^^^^^^
note: required because it's used within this closure
   --> compiler/rustc_query_impl/src/plumbing.rs:813:25
    |
813 |                 s.spawn(move || {
    |                         ^^^^^^^
note: required by a bound in `std::thread::Scope::<'scope, 'env>::spawn`
    |
191 |         F: FnOnce() -> T + Send + 'scope,
191 |         F: FnOnce() -> T + Send + 'scope,
