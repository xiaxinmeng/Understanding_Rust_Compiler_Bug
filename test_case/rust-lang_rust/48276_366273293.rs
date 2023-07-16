
error: internal compiler error: /checkout/src/librustc/infer/region_constraints/mod.rs:685: cannot relate bound region: ReStatic <= ReLateBound(DebruijnIndex { depth: 1 }, BrAnon(0))

thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc_errors/lib.rs:451:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at /checkout/src/libstd/sys_common/backtrace.rs:68
             at /checkout/src/libstd/sys_common/backtrace.rs:57
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
   6: rustc_errors::Handler::span_bug
   7: <std::thread::local::LocalKey<T>>::with
   8: rustc::ty::context::tls::with_opt
   9: rustc::session::opt_span_bug_fmt
  10: rustc::session::span_bug_fmt
  11: rustc::infer::region_constraints::RegionConstraintCollector::make_subregion
  12: rustc::infer::region_constraints::RegionConstraintCollector::make_eqregion
  13: <rustc::infer::equate::Equate<'combine, 'infcx, 'gcx, 'tcx> as rustc::ty::relate::TypeRelation<'infcx, 'gcx, 'tcx>>::regions
  14: rustc::ty::relate::super_relate_tys
  15: rustc::infer::combine::<impl rustc::infer::InferCtxt<'infcx, 'gcx, 'tcx>>::super_combine_tys
  16: <rustc::infer::equate::Equate<'combine, 'infcx, 'gcx, 'tcx> as rustc::ty::relate::TypeRelation<'infcx, 'gcx, 'tcx>>::tys
  17: rustc::infer::InferCtxt::commit_if_ok
  18: rustc::infer::InferCtxt::can_eq
  19: rustc::ty::util::ExplicitSelf::determine
  20: <std::thread::local::LocalKey<T>>::with
  21: rustc::ty::context::GlobalCtxt::enter_local
  22: rustc_typeck::check::compare_method::compare_self_type::{{closure}}
  23: rustc_typeck::check::compare_method::compare_impl_method
  24: rustc_typeck::check::check_item_type
  25: rustc::hir::Crate::visit_all_item_likes
  26: rustc_typeck::check_crate
  27: <std::thread::local::LocalKey<T>>::with
  28: <std::thread::local::LocalKey<T>>::with
  29: rustc::ty::context::TyCtxt::create_and_enter
  30: rustc_driver::driver::compile_input
  31: rustc_driver::run_compiler
