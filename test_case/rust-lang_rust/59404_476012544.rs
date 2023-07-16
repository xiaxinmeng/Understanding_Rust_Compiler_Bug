plain
travis_time:end:056d2388:start=1553461630682769544,finish=1553461704399551723,duration=73716782179
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:50]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:04:15]    Compiling synstructure v0.10.1
[00:04:21]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:04:30]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:04:56] error[E0277]: `std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>` cannot be shared between threads safely
[00:04:56]      |
[00:04:56]      |
[00:04:56] 2739 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
[00:04:56]      |                                                ^^^^^^^^ `std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>` cannot be shared between threads safely
[00:04:56]      |
[00:04:56]      = help: within `std::option::Option<std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>>`, the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>`
[00:04:56]      = note: required because it appears within the type `std::option::Option<std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>>`
[00:04:56]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, std::option::Option<std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>>>`
[00:04:56]      = note: required because it appears within the type `rustc_data_structures::sync::RwLock<std::option::Option<std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>>>`
[00:04:56]      = note: required because it appears within the type `ty::steal::Steal<std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>>`
[00:04:56]      = note: required because it appears within the type `ty::context::GlobalCtxt<'gcx>`
[00:04:56]      = note: required because it appears within the type `&'gcx ty::context::GlobalCtxt<'gcx>`
[00:04:56]      = note: required because it appears within the type `ty::context::TyCtxt<'_, 'gcx, 'tcx>`
[00:04:56]      = note: required because it appears within the type `&ty::context::TyCtxt<'_, 'gcx, 'tcx>`
[00:04:56]      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2739:57: 2741:10 f:&F, self:&ty::context::TyCtxt<'_, 'gcx, 'tcx>]`
[00:04:56] 
[00:04:56] error[E0277]: `(dyn std::any::Any + 'static)` cannot be sent between threads safely
[00:04:56]      |
[00:04:56]      |
[00:04:56] 2739 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
[00:04:56]      |                                                ^^^^^^^^ `(dyn std::any::Any + 'static)` cannot be sent between threads safely
[00:04:56]      |
[00:04:56]      = help: the trait `std::marker::Send` is not implemented for `(dyn std::any::Any + 'static)`
[00:04:56]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::any::Any + 'static)>`
[00:04:56]      = note: required because it appears within the type `std::boxed::Box<(dyn std::any::Any + 'static)>`
[00:04:56]      = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>`
[00:04:56]      = note: required because of the requirements on the impl of `std::marker::Send` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>>`
[00:04:56]      = note: required because it appears within the type `rustc_data_structures::sync::RwLock<std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>>`
[00:04:56]      = note: required because it appears within the type `ty::steal::Steal<std::boxed::Box<(dyn std::any::Any + 'static)>>`
[00:04:56]      = note: required because it appears within the type `ty::OngoingCodegen`
[00:04:56]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<ty::OngoingCodegen>`
[00:04:56]      = note: required because it appears within the type `std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>`
[00:04:56]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>`
[00:04:56]      = note: required because it appears within the type `(hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>)`
[00:04:56]      = note: required because it appears within the type `std::marker::PhantomData<(hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>)>`
[00:04:56]      = note: required because it appears within the type `std::collections::hash::table::RawTable<hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>>`
[00:04:56]      = note: required because it appears within the type `std::collections::HashMap<hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:04:56]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'gcx, ty::query::queries::ongoing_codegen<'gcx>>`
[00:04:56]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'gcx, ty::query::queries::ongoing_codegen<'gcx>>>`
[00:04:56]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'gcx, ty::query::queries::ongoing_codegen<'gcx>>>`
[00:04:56]      = note: required because it appears within the type `ty::query::Queries<'gcx>`
[00:04:56]      = note: required because it appears within the type `ty::context::GlobalCtxt<'gcx>`
[00:04:56]      = note: required because it appears within the type `&'gcx ty::context::GlobalCtxt<'gcx>`
[00:04:56]      = note: required because it appears within the type `ty::context::TyCtxt<'_, 'gcx, 'tcx>`
[00:04:56]      = note: required because it appears within the type `&ty::context::TyCtxt<'_, 'gcx, 'tcx>`
[00:04:56]      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2739:57: 2741:10 f:&F, self:&ty::context::TyCtxt<'_, 'gcx, 'tcx>]`
[00:04:56] 
[00:04:56] error[E0277]: `(dyn std::any::Any + 'static)` cannot be shared between threads safely
[00:04:56]      |
[00:04:56]      |
[00:04:56] 2739 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
[00:04:56]      |                                                ^^^^^^^^ `(dyn std::any::Any + 'static)` cannot be shared between threads safely
[00:04:56]      |
[00:04:56]      = help: the trait `std::marker::Sync` is not implemented for `(dyn std::any::Any + 'static)`
[00:04:56]      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(dyn std::any::Any + 'static)>`
[00:04:56]      = note: required because it appears within the type `std::boxed::Box<(dyn std::any::Any + 'static)>`
[00:04:56]      = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>`
[00:04:56]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>>`
[00:04:56]      = note: required because it appears within the type `rustc_data_structures::sync::RwLock<std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>>`
[00:04:56]      = note: required because it appears within the type `ty::steal::Steal<std::boxed::Box<(dyn std::any::Any + 'static)>>`
[00:04:56]      = note: required because it appears within the type `ty::OngoingCodegen`
[00:04:56]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<ty::OngoingCodegen>`
[00:04:56]      = note: required because it appears within the type `std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>`
[00:04:56]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>`
[00:04:56]      = note: required because it appears within the type `(hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>)`
[00:04:56]      = note: required because it appears within the type `std::marker::PhantomData<(hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>)>`
[00:04:56]      = note: required because it appears within the type `std::collections::hash::table::RawTable<hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>>`
[00:04:56]      = note: required because it appears within the type `std::collections::HashMap<hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:04:56]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'gcx, ty::query::queries::ongoing_codegen<'gcx>>`
[00:04:56]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'gcx, ty::query::queries::ongoing_codegen<'gcx>>>`
[00:04:56]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'gcx, ty::query::queries::ongoing_codegen<'gcx>>>`
[00:04:56]      = note: required because it appears within the type `ty::query::Queries<'gcx>`
[00:04:56]      = note: required because it appears within the type `ty::context::GlobalCtxt<'gcx>`
[00:04:56]      = note: required because it appears within the type `&'gcx ty::context::GlobalCtxt<'gcx>`
[00:04:56]      = note: required because it appears within the type `ty::context::TyCtxt<'_, 'gcx, 'tcx>`
[00:04:56]      = note: required because it appears within the type `&ty::context::TyCtxt<'_, 'gcx, 'tcx>`
[00:04:56]      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2739:57: 2741:10 f:&F, self:&ty::context::TyCtxt<'_, 'gcx, 'tcx>]`
[00:04:56] 
[00:05:00] error[E0277]: `std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>` cannot be shared between threads safely
[00:05:00]     --> src/librustc/ty/context.rs:2015:13
[00:05:00]      |
[00:05:00] 2015 |             sync::assert_sync::<ImplicitCtxt<'_, '_, '_>>();
[00:05:00]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>` cannot be shared between threads safely
[00:05:00]      |
[00:05:00]      = help: within `std::option::Option<std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>>`, the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>`
[00:05:00]      = note: required because it appears within the type `std::option::Option<std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>>`
[00:05:00]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, std::option::Option<std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>>>`
[00:05:00]      = note: required because it appears within the type `rustc_data_structures::sync::RwLock<std::option::Option<std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>>>`
[00:05:00]      = note: required because it appears within the type `ty::steal::Steal<std::sync::mpsc::Receiver<std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>>`
[00:05:00]      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
[00:05:00]      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
[00:05:00]      = note: required because it appears within the type `ty::context::TyCtxt<'_, '_, '_>`
[00:05:00]      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_, '_>`
[00:05:00]      = note: required by `rustc_data_structures::sync::assert_sync`
[00:05:00] 
[00:05:00] error[E0277]: `(dyn std::any::Any + 'static)` cannot be sent between threads safely
[00:05:00]     --> src/librustc/ty/context.rs:2015:13
[00:05:00]      |
[00:05:00] 2015 |             sync::assert_sync::<ImplicitCtxt<'_, '_, '_>>();
[00:05:00]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn std::any::Any + 'static)` cannot be sent between threads safely
[00:05:00]      |
[00:05:00]      = help: the trait `std::marker::Send` is not implemented for `(dyn std::any::Any + 'static)`
[00:05:00]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn std::any::Any + 'static)>`
[00:05:00]      = note: required because it appears within the type `std::boxed::Box<(dyn std::any::Any + 'static)>`
[00:05:00]      = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>`
[00:05:00]      = note: required because of the requirements on the impl of `std::marker::Send` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>>`
[00:05:00]      = note: required because it appears within the type `rustc_data_structures::sync::RwLock<std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>>`
[00:05:00]      = note: required because it appears within the type `ty::steal::Steal<std::boxed::Box<(dyn std::any::Any + 'static)>>`
[00:05:00]      = note: required because it appears within the type `ty::OngoingCodegen`
[00:05:00]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<ty::OngoingCodegen>`
[00:05:00]      = note: required because it appears within the type `std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>`
[00:05:00]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>`
[00:05:00]      = note: required because it appears within the type `(hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>)`
[00:05:00]      = note: required because it appears within the type `std::marker::PhantomData<(hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>)>`
[00:05:00]      = note: required because it appears within the type `std::collections::hash::table::RawTable<hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>>`
[00:05:00]      = note: required because it appears within the type `std::collections::HashMap<hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:05:00]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'_, ty::query::queries::ongoing_codegen<'_>>`
[00:05:00]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'_, ty::query::queries::ongoing_codegen<'_>>>`
[00:05:00]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'_, ty::query::queries::ongoing_codegen<'_>>>`
[00:05:00]      = note: required because it appears within the type `ty::query::Queries<'_>`
[00:05:00]      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
[00:05:00]      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
[00:05:00]      = note: required because it appears within the type `ty::context::TyCtxt<'_, '_, '_>`
[00:05:00]      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_, '_>`
[00:05:00]      = note: required by `rustc_data_structures::sync::assert_sync`
[00:05:00] 
[00:05:00] error[E0277]: `(dyn std::any::Any + 'static)` cannot be shared between threads safely
[00:05:00]     --> src/librustc/ty/context.rs:2015:13
[00:05:00]      |
[00:05:00] 2015 |             sync::assert_sync::<ImplicitCtxt<'_, '_, '_>>();
[00:05:00]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn std::any::Any + 'static)` cannot be shared between threads safely
[00:05:00]      |
[00:05:00]      = help: the trait `std::marker::Sync` is not implemented for `(dyn std::any::Any + 'static)`
[00:05:00]      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(dyn std::any::Any + 'static)>`
[00:05:00]      = note: required because it appears within the type `std::boxed::Box<(dyn std::any::Any + 'static)>`
[00:05:00]      = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>`
[00:05:00]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>>`
[00:05:00]      = note: required because it appears within the type `rustc_data_structures::sync::RwLock<std::option::Option<std::boxed::Box<(dyn std::any::Any + 'static)>>>`
[00:05:00]      = note: required because it appears within the type `ty::steal::Steal<std::boxed::Box<(dyn std::any::Any + 'static)>>`
[00:05:00]      = note: required because it appears within the type `ty::OngoingCodegen`
[00:05:00]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<ty::OngoingCodegen>`
[00:05:00]      = note: required because it appears within the type `std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>`
[00:05:00]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>`
[00:05:00]      = note: required because it appears within the type `(hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>)`
[00:05:00]      = note: required because it appears within the type `std::marker::PhantomData<(hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>)>`
[00:05:00]      = note: required because it appears within the type `std::collections::hash::table::RawTable<hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>>`
[00:05:00]      = note: required because it appears within the type `std::collections::HashMap<hir::def_id::CrateNum, ty::query::plumbing::QueryValue<std::result::Result<std::sync::Arc<ty::OngoingCodegen>, util::common::ErrorReported>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:05:00]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'_, ty::query::queries::ongoing_codegen<'_>>`
[00:05:00]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'_, ty::query::queries::ongoing_codegen<'_>>>`
[00:05:00]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'_, ty::query::queries::ongoing_codegen<'_>>>`
[00:05:00]      = note: required because it appears within the type `ty::query::Queries<'_>`
[00:05:00]      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
[00:05:00]      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
[00:05:00]      = note: required because it appears within the type `ty::context::TyCtxt<'_, '_, '_>`
[00:05:00]      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_, '_>`
[00:05:00]      = note: required by `rustc_data_structures::sync::assert_sync`
[00:05:02] error: aborting due to 6 previous errors
[00:05:02] 
[00:05:02] For more information about this error, try `rustc --explain E0277`.
[00:05:02] error: Could not compile `rustc`.
---
travis_time:end:05f6737c:start=1553462017254880580,finish=1553462017261568521,duration=6687941
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05c49738
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00e6ce9b
travis_time:start:00e6ce9b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:270408e0
$ dmesg | grep -i kill
