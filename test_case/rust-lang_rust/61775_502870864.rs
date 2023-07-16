plain
travis_time:end:0d6fda7c:start=1560803083624994072,finish=1560803086050584147,duration=2425590075
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:05:55]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:57]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:57]     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:06:57]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:17] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:07:17]      |
[00:07:17] 1494 |     join(|| {
[00:07:17] 1494 |     join(|| {
[00:07:17]      |     ^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:07:17]      |
[00:07:17]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:07:17]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:07:17]      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:07:17]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:07:17]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:07:17]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:07:17]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>`
[00:07:17]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:07:17]      = note: required because of the requirements on the impl of `std::marker::Send` for `&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:07:17]      = note: required because it appears within the type `std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>`
[00:07:17]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>`
[00:07:17]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)`
[00:07:17]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)>`
[00:07:17]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:17]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:17]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'tcx, ty::query::queries::normalize_projection_ty<'tcx>>`
[00:07:17]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'tcx, ty::query::queries::normalize_projection_ty<'tcx>>>`
[00:07:17]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'tcx, ty::query::queries::normalize_projection_ty<'tcx>>>`
[00:07:17]      = note: required because it appears within the type `ty::query::Queries<'tcx>`
[00:07:17]      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
[00:07:17]      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
[00:07:17]      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
[00:07:17]      = note: required because of the requirements on the impl of `std::marker::Send` for `&ty::context::TyCtxt<'tcx>`
[00:07:17]      = note: required because it appears within the type `[closure@src/librustc/lint/context.rs:1494:10: 1499:6 tcx:&ty::context::TyCtxt<'tcx>, builtin_lints:impl FnOnce() -> T + Send]`
[00:07:17]      = note: required by `rustc_rayon_core::join::join`
[00:07:17] 
[00:07:17] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:07:17]      |
[00:07:17] 1494 |     join(|| {
[00:07:17] 1494 |     join(|| {
[00:07:17]      |     ^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:07:17]      |
[00:07:17]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:07:17]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:07:17]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:07:17]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:07:17]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:07:17]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:07:17]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, &ty::TyS<'_>>`
[00:07:17]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, &ty::TyS<'_>>>`
[00:07:17]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefBadTy<'_>`
[00:07:17]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>`
[00:07:17]      = note: required because it appears within the type `std::option::Option<std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>>`
[00:07:17]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefStepsResult<'_>`
[00:07:17]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>`
[00:07:17]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)`
[00:07:17]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)>`
[00:07:17]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:17]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:17]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'tcx, ty::query::queries::method_autoderef_steps<'tcx>>`
[00:07:17]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'tcx, ty::query::queries::method_autoderef_steps<'tcx>>>`
[00:07:17]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'tcx, ty::query::queries::method_autoderef_steps<'tcx>>>`
[00:07:17]      = note: required because it appears within the type `ty::query::Queries<'tcx>`
[00:07:17]      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
[00:07:17]      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
[00:07:17]      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
[00:07:17]      = note: required because of the requirements on the impl of `std::marker::Send` for `&ty::context::TyCtxt<'tcx>`
[00:07:17]      = note: required because it appears within the type `[closure@src/librustc/lint/context.rs:1494:10: 1499:6 tcx:&ty::context::TyCtxt<'tcx>, builtin_lints:impl FnOnce() -> T + Send]`
[00:07:17]      = note: required by `rustc_rayon_core::join::join`
[00:07:17] 
[00:07:23] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:07:23]      |
[00:07:23]      |
[00:07:23] 2775 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
[00:07:23]      |                                                ^^^^^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:07:23]      |
[00:07:23]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:07:23]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:07:23]      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:07:23]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:07:23]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:07:23]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:07:23]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>`
[00:07:23]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:07:23]      = note: required because of the requirements on the impl of `std::marker::Send` for `&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:07:23]      = note: required because it appears within the type `std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>`
[00:07:23]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>`
[00:07:23]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)`
[00:07:23]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)>`
[00:07:23]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:23]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:23]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'tcx, ty::query::queries::normalize_projection_ty<'tcx>>`
[00:07:23]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'tcx, ty::query::queries::normalize_projection_ty<'tcx>>>`
[00:07:23]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'tcx, ty::query::queries::normalize_projection_ty<'tcx>>>`
[00:07:23]      = note: required because it appears within the type `ty::query::Queries<'tcx>`
[00:07:23]      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
[00:07:23]      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
[00:07:23]      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
[00:07:23]      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
[00:07:23]      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2775:57: 2777:10 f:&F, self:&ty::context::TyCtxt<'tcx>]`
[00:07:23] 
[00:07:23] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:07:23]      |
[00:07:23]      |
[00:07:23] 2775 |         par_iter(&self.hir().krate().body_ids).for_each(|&body_id| {
[00:07:23]      |                                                ^^^^^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:07:23]      |
[00:07:23]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:07:23]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:07:23]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:07:23]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:07:23]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:07:23]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:07:23]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, &ty::TyS<'_>>`
[00:07:23]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, &ty::TyS<'_>>>`
[00:07:23]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefBadTy<'_>`
[00:07:23]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>`
[00:07:23]      = note: required because it appears within the type `std::option::Option<std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>>`
[00:07:23]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefStepsResult<'_>`
[00:07:23]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>`
[00:07:23]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)`
[00:07:23]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)>`
[00:07:23]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:23]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:23]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'tcx, ty::query::queries::method_autoderef_steps<'tcx>>`
[00:07:23]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'tcx, ty::query::queries::method_autoderef_steps<'tcx>>>`
[00:07:23]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'tcx, ty::query::queries::method_autoderef_steps<'tcx>>>`
[00:07:23]      = note: required because it appears within the type `ty::query::Queries<'tcx>`
[00:07:23]      = note: required because it appears within the type `ty::context::GlobalCtxt<'tcx>`
[00:07:23]      = note: required because it appears within the type `&'tcx ty::context::GlobalCtxt<'tcx>`
[00:07:23]      = note: required because it appears within the type `ty::context::TyCtxt<'tcx>`
[00:07:23]      = note: required because it appears within the type `&ty::context::TyCtxt<'tcx>`
[00:07:23]      = note: required because it appears within the type `[closure@src/librustc/ty/mod.rs:2775:57: 2777:10 f:&F, self:&ty::context::TyCtxt<'tcx>]`
[00:07:23] 
[00:07:27] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:07:27]     --> src/librustc/ty/context.rs:2011:13
[00:07:27]      |
[00:07:27] 2011 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
[00:07:27]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be shared between threads safely
[00:07:27]      |
[00:07:27]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:07:27]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:07:27]      = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:07:27]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:07:27]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:07:27]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:07:27]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>`
[00:07:27]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:07:27]      = note: required because of the requirements on the impl of `std::marker::Send` for `&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>`
[00:07:27]      = note: required because it appears within the type `std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>`
[00:07:27]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>`
[00:07:27]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)`
[00:07:27]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>)>`
[00:07:27]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:27]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, ty::sty::ProjectionTy<'_>>>, ty::query::plumbing::QueryValue<std::result::Result<&infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, traits::query::normalize::NormalizationResult<'_>>>, traits::query::NoSolution>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:27]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'_, ty::query::queries::normalize_projection_ty<'_>>`
[00:07:27]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'_, ty::query::queries::normalize_projection_ty<'_>>>`
[00:07:27]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'_, ty::query::queries::normalize_projection_ty<'_>>>`
[00:07:27]      = note: required because it appears within the type `ty::query::Queries<'_>`
[00:07:27]      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
[00:07:27]      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
[00:07:27]      = note: required because it appears within the type `ty::context::TyCtxt<'_>`
[00:07:27]      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_>`
[00:07:27]      = note: required by `rustc_data_structures::sync::assert_sync`
[00:07:27] 
[00:07:27] error[E0277]: `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:07:27]     --> src/librustc/ty/context.rs:2011:13
[00:07:27]      |
[00:07:27] 2011 |             sync::assert_sync::<ImplicitCtxt<'_, '_>>();
[00:07:27]      |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>` cannot be sent between threads safely
[00:07:27]      |
[00:07:27]      = help: within `infer::region_constraints::PickConstraint<'_>`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<std::vec::Vec<&ty::sty::RegionKind>>`
[00:07:27]      = note: required because it appears within the type `infer::region_constraints::PickConstraint<'_>`
[00:07:27]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<infer::region_constraints::PickConstraint<'_>>`
[00:07:27]      = note: required because it appears within the type `alloc::raw_vec::RawVec<infer::region_constraints::PickConstraint<'_>>`
[00:07:27]      = note: required because it appears within the type `std::vec::Vec<infer::region_constraints::PickConstraint<'_>>`
[00:07:27]      = note: required because it appears within the type `infer::canonical::QueryRegionConstraints<'_>`
[00:07:27]      = note: required because it appears within the type `infer::canonical::QueryResponse<'_, &ty::TyS<'_>>`
[00:07:27]      = note: required because it appears within the type `infer::canonical::Canonical<'_, infer::canonical::QueryResponse<'_, &ty::TyS<'_>>>`
[00:07:27]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefBadTy<'_>`
[00:07:27]      = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>`
[00:07:27]      = note: required because it appears within the type `std::option::Option<std::sync::Arc<traits::query::method_autoderef::MethodAutoderefBadTy<'_>>>`
[00:07:27]      = note: required because it appears within the type `traits::query::method_autoderef::MethodAutoderefStepsResult<'_>`
[00:07:27]      = note: required because it appears within the type `ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>`
[00:07:27]      = note: required because it appears within the type `(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)`
[00:07:27]      = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>)>`
[00:07:27]      = note: required because it appears within the type `hashbrown::map::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:27]      = note: required because it appears within the type `std::collections::HashMap<infer::canonical::Canonical<'_, ty::ParamEnvAnd<'_, &ty::TyS<'_>>>, ty::query::plumbing::QueryValue<traits::query::method_autoderef::MethodAutoderefStepsResult<'_>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[00:07:27]      = note: required because it appears within the type `ty::query::plumbing::QueryCache<'_, ty::query::queries::method_autoderef_steps<'_>>`
[00:07:27]      = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, ty::query::plumbing::QueryCache<'_, ty::query::queries::method_autoderef_steps<'_>>>`
[00:07:27]      = note: required because it appears within the type `rustc_data_structures::sync::Lock<ty::query::plumbing::QueryCache<'_, ty::query::queries::method_autoderef_steps<'_>>>`
[00:07:27]      = note: required because it appears within the type `ty::query::Queries<'_>`
[00:07:27]      = note: required because it appears within the type `ty::context::GlobalCtxt<'_>`
[00:07:27]      = note: required because it appears within the type `&ty::context::GlobalCtxt<'_>`
[00:07:27]      = note: required because it appears within the type `ty::context::TyCtxt<'_>`
[00:07:27]      = note: required because it appears within the type `ty::context::tls::ImplicitCtxt<'_, '_>`
[00:07:27]      = note: required by `rustc_data_structures::sync::assert_sync`
[00:07:29] error: aborting due to 6 previous errors
[00:07:29] 
[00:07:29] For more information about this error, try `rustc --explain E0277`.
[00:07:29] error: Could not compile `rustc`.
---
travis_time:end:01036a82:start=1560803547925319315,finish=1560803547930409918,duration=5090603
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04b30965
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0364d39a
travis_time:start:0364d39a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2d50d79f
$ dmesg | grep -i kill
