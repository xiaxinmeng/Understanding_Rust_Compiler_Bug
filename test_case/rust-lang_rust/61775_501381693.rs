plain
travis_time:end:04baf64a:start=1560354730290925912,finish=1560354732605148173,duration=2314222261
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:27]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:28]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:28]     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:06:20]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:37] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:06:37]      |
[00:06:37] 1500 |     join(|| {
[00:06:37] 1500 |     join(|| {
[00:06:37]      |     ^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:06:37]      |
[00:06:37]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:06:37]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:06:37]      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:06:37]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:06:37]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:06:37]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:06:37]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>`
[00:06:37]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:06:37]      = note: required because of the requirements on the impl of `std::marker::Send` for `&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:06:37]      = note: required because it appears within the type `std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>`
[00:06:37]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>`
[00:06:37]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)`
[00:06:37]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)>`
[00:06:37]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:37]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:37]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'tcx, ty::query::queries::normalize_projection_ty<'tcx>>`
[00:06:37]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'tcx, ty::query::queries::normalize_projection_ty<'tcx>>>`
[00:06:37]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'tcx, ty::query::queries::normalize_projection_ty<'tcx>>>`
[00:06:37]      = note: required because it appears within the type `ty::query::Queries<'tcx>`
[00:06:37]      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
[00:06:37]      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
[00:06:37]      = note: required because it appears within the type `ty::context::TyCtxt<'_, 'tcx, '_>`
[00:06:37]      = note: required because of the requirements on the impl of `std::marker::Send` for `&ty::context::TyCtxt<'_, 'tcx, '_>`
[00:06:37]      = note: required because it appears within the type `[closure@src/librustc/lint/context.rs:1500:10: 1505:6 tcx:&ty::context::TyCtxt<'_, 'tcx, '_>, builtin_lints:impl FnOnce() -> T + Send]`
[00:06:37]      = note: required by `rustc_rayon_core::join::join`
[00:06:37] 
[00:06:37] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:06:37]      |
[00:06:37] 1500 |     join(|| {
[00:06:37] 1500 |     join(|| {
[00:06:37]      |     ^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:06:37]      |
[00:06:37]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:06:37]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:06:37]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:06:37]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:06:37]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:06:37]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:06:37]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, &ty::TyS<'_>>`
[00:06:37]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, &ty::TyS<'_>>>`
[00:06:37]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefBadTy<'_>`
[00:06:37]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>`
[00:06:37]      = note: required because it appears within the type `std::option::Option<std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>>`
[00:06:37]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefStepsResult<'_>`
[00:06:37]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>`
[00:06:37]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)`
[00:06:37]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)>`
[00:06:37]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:37]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:37]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'tcx, ty::query::queries::method_autoderef_steps<'tcx>>`
[00:06:37]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'tcx, ty::query::queries::method_autoderef_steps<'tcx>>>`
[00:06:37]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'tcx, ty::query::queries::method_autoderef_steps<'tcx>>>`
[00:06:37]      = note: required because it appears within the type `ty::query::Queries<'tcx>`
[00:06:37]      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
[00:06:37]      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
[00:06:37]      = note: required because it appears within the type `ty::context::TyCtxt<'_, 'tcx, '_>`
[00:06:37]      = note: required because of the requirements on the impl of `std::marker::Send` for `&ty::context::TyCtxt<'_, 'tcx, '_>`
[00:06:37]      = note: required because it appears within the type `[closure@src/librustc/lint/context.rs:1500:10: 1505:6 tcx:&ty::context::TyCtxt<'_, 'tcx, '_>, builtin_lints:impl FnOnce() -> T + Send]`
[00:06:37]      = note: required by `rustc_rayon_core::join::join`
[00:06:37] 
[00:06:42] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:06:42]      |
[00:06:42]      |
[00:06:42] 2776 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
[00:06:42]      |                                                ^^^^^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:06:42]      |
[00:06:42]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:06:42]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:06:42]      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:06:42]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:06:42]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:06:42]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:06:42]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>`
[00:06:42]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:06:42]      = note: required because of the requirements on the impl of `std::marker::Send` for `&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:06:42]      = note: required because it appears within the type `std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>`
[00:06:42]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>`
[00:06:42]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)`
[00:06:42]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)>`
[00:06:42]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:42]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:42]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'gcx, ty::query::queries::normalize_projection_ty<'gcx>>`
[00:06:42]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'gcx, ty::query::queries::normalize_projection_ty<'gcx>>>`
[00:06:42]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'gcx, ty::query::queries::normalize_projection_ty<'gcx>>>`
[00:06:42]      = note: required because it appears within the type `ty::query::Queries<'gcx>`
[00:06:42]      = note: required because it appears within the type `ty::context::GlobalCtxt<'gcx>`
[00:06:42]      = note: required because it appears within the type `&'gcx ty::context::GlobalCtxt<'gcx>`
[00:06:42]      = note: required because it appears within the type `ty::context::TyCtxt<'_, 'gcx, '_>`
[00:06:42]      = note: required because it appears within the type `&ty::context::TyCtxt<'_, 'gcx, '_>`
[00:06:42]      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2776:57: 2778:10 f:&F, self:&ty::context::TyCtxt<'_, 'gcx, '_>]`
[00:06:42] 
[00:06:42] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:06:42]      |
[00:06:42]      |
[00:06:42] 2776 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
[00:06:42]      |                                                ^^^^^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:06:42]      |
[00:06:42]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:06:42]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:06:42]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:06:42]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:06:42]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:06:42]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:06:42]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, &ty::TyS<'_>>`
[00:06:42]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, &ty::TyS<'_>>>`
[00:06:42]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefBadTy<'_>`
[00:06:42]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>`
[00:06:42]      = note: required because it appears within the type `std::option::Option<std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>>`
[00:06:42]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefStepsResult<'_>`
[00:06:42]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>`
[00:06:42]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)`
[00:06:42]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)>`
[00:06:42]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:42]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:42]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'gcx, ty::query::queries::method_autoderef_steps<'gcx>>`
[00:06:42]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'gcx, ty::query::queries::method_autoderef_steps<'gcx>>>`
[00:06:42]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'gcx, ty::query::queries::method_autoderef_steps<'gcx>>>`
[00:06:42]      = note: required because it appears within the type `ty::query::Queries<'gcx>`
[00:06:42]      = note: required because it appears within the type `ty::context::GlobalCtxt<'gcx>`
[00:06:42]      = note: required because it appears within the type `&'gcx ty::context::GlobalCtxt<'gcx>`
[00:06:42]      = note: required because it appears within the type `ty::context::TyCtxt<'_, 'gcx, '_>`
[00:06:42]      = note: required because it appears within the type `&ty::context::TyCtxt<'_, 'gcx, '_>`
[00:06:42]      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2776:57: 2778:10 f:&F, self:&ty::context::TyCtxt<'_, 'gcx, '_>]`
[00:06:42] 
[00:06:45] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:06:45]     --> src/librustc/ty/context.rs:2012:13
[00:06:45]      |
[00:06:45] 2012 |             sync::assert_sync::<ImplicitCtxt<'_, '_, '_>>();
[00:06:45]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:06:45]      |
[00:06:45]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:06:45]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:06:45]      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:06:45]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:06:45]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:06:45]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:06:45]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>`
[00:06:45]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:06:45]      = note: required because of the requirements on the impl of `std::marker::Send` for `&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:06:45]      = note: required because it appears within the type `std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>`
[00:06:45]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>`
[00:06:45]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)`
[00:06:45]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)>`
[00:06:45]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:45]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:45]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'_, ty::query::queries::normalize_projection_ty<'_>>`
[00:06:45]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'_, ty::query::queries::normalize_projection_ty<'_>>>`
[00:06:45]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'_, ty::query::queries::normalize_projection_ty<'_>>>`
[00:06:45]      = note: required because it appears within the type `ty::query::Queries<'_>`
[00:06:45]      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
[00:06:45]      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
[00:06:45]      = note: required because it appears within the type `ty::context::TyCtxt<'_, '_, '_>`
[00:06:45]      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_, '_>`
[00:06:45]      = note: required by `rustc_data_structures::sync::assert_sync`
[00:06:45] 
[00:06:45] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:06:45]     --> src/librustc/ty/context.rs:2012:13
[00:06:45]      |
[00:06:45] 2012 |             sync::assert_sync::<ImplicitCtxt<'_, '_, '_>>();
[00:06:45]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:06:45]      |
[00:06:45]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:06:45]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:06:45]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:06:45]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:06:45]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:06:45]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:06:45]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, &ty::TyS<'_>>`
[00:06:45]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, &ty::TyS<'_>>>`
[00:06:45]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefBadTy<'_>`
[00:06:45]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>`
[00:06:45]      = note: required because it appears within the type `std::option::Option<std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>>`
[00:06:45]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefStepsResult<'_>`
[00:06:45]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>`
[00:06:45]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)`
[00:06:45]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)>`
[00:06:45]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:45]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:06:45]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'_, ty::query::queries::method_autoderef_steps<'_>>`
[00:06:45]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'_, ty::query::queries::method_autoderef_steps<'_>>>`
[00:06:45]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'_, ty::query::queries::method_autoderef_steps<'_>>>`
[00:06:45]      = note: required because it appears within the type `ty::query::Queries<'_>`
[00:06:45]      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
[00:06:45]      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
[00:06:45]      = note: required because it appears within the type `ty::context::TyCtxt<'_, '_, '_>`
[00:06:45]      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_, '_>`
[00:06:45]      = note: required by `rustc_data_structures::sync::assert_sync`
[00:06:46] error: aborting due to 6 previous errors
[00:06:46] 
[00:06:46] For more information about this error, try `rustc --explain E0277`.
[00:06:47] error: Could not compile `rustc`.
---
travis_time:end:0488e6c9:start=1560355151751656374,finish=1560355151757342798,duration=5686424
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:31bfd8ee
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03d67f32
travis_time:start:03d67f32
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06047961
$ dmesg | grep -i kill
