
⋊> /t/r/openssl on master  env RUSTFLAGS='-C link-dead-code' cargo build
   Compiling openssl v0.10.2 (file:///tmp/rust-openssl/openssl)
error: internal compiler error: librustc/ty/subst.rs:424: Region parameter out of range when substituting in region 'a (root type=Some(unsafe fn(*mut <Self as foreign_types::ForeignTypeRef>::CType) -> &'a mut Self {<Self as foreign_types::ForeignTypeRef>::from_ptr_mut::<'a>})) (index=1)

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-nightly (5965b7901 2018-01-19) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:456:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:68
             at libstd/sys_common/backtrace.rs:57
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:380
   3: std::panicking::default_hook
             at libstd/panicking.rs:390
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:576
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::span_bug
   7: <std::thread::local::LocalKey<T>>::with
   8: rustc::ty::context::tls::with_opt
   9: rustc::session::opt_span_bug_fmt
  10: rustc::session::span_bug_fmt
  11: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_region
  12: rustc::ty::fold::TypeFoldable::fold_with
  13: <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter
  14: rustc::ty::fold::TypeFoldable::fold_with
  15: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  16: <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  17: rustc::traits::trans::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'tcx>>::trans_apply_param_substs
  18: rustc::ty::instance::Instance::ty
  19: rustc_trans::trans_item::predefine_fn
  20: rustc_trans::trans_item::MonoItemExt::predefine
  21: rustc_trans::base::compile_codegen_unit
  22: rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::compute_result
  23: rustc::dep_graph::graph::DepGraph::with_task_impl
  24: rustc_errors::Handler::track_diagnostics
  25: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  26: rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::force
  27: rustc::ty::maps::<impl rustc::ty::maps::queries::compile_codegen_unit<'tcx>>::try_get
  28: rustc::ty::maps::TyCtxtAt::compile_codegen_unit
  29: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::compile_codegen_unit
  30: rustc_trans::base::trans_crate
  31: <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate
  32: rustc::util::common::time
  33: rustc_driver::driver::phase_4_translate_to_llvm
  34: rustc_driver::driver::compile_input::{{closure}}
  35: <std::thread::local::LocalKey<T>>::with
  36: <std::thread::local::LocalKey<T>>::with
  37: rustc::ty::context::TyCtxt::create_and_enter
  38: rustc_driver::driver::compile_input
  39: rustc_driver::run_compiler

error: Could not compile `openssl`.
