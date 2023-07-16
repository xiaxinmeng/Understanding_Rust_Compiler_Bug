
error: internal compiler error: librustc/infer/error_reporting/mod.rs:184: impossible case reached

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:207
   3: std::panicking::default_hook
             at libstd/panicking.rs:223
   4: core::ops::function::Fn::call
   5: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:403
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::session::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: <std::thread::local::LocalKey<T>>::try_with
  11: <std::thread::local::LocalKey<T>>::with
  12: rustc::ty::context::tls::with
  13: rustc::ty::context::tls::with_opt
  14: rustc::session::opt_span_bug_fmt
  15: rustc::session::bug_fmt
  16: rustc::infer::error_reporting::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::msg_span_from_free_region
  17: <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_region
  18: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  19: <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  20: <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter
  21: rustc::ty::fold::TypeFoldable::fold_with
  22: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  23: <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  24: <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter
  25: rustc::ty::fold::TypeFoldable::fold_with
  26: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  27: <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  28: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &'a mut F>::call_once
  29: <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter
  30: <T as rustc::ty::context::InternIteratorElement<T, R>>::intern_with
  31: <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  32: <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter
  33: rustc::ty::fold::TypeFoldable::fold_with
  34: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  35: <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  36: <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter
  37: rustc::ty::fold::TypeFoldable::fold_with
  38: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  39: <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  40: <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter
  41: rustc::ty::fold::TypeFoldable::fold_with
  42: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  43: <rustc::infer::anon_types::ReverseMapper<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  44: rustc::infer::anon_types::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::infer_anon_definition_from_instantiation
  45: rustc_typeck::check::writeback::WritebackCx::visit_anon_types
  46: rustc_typeck::check::writeback::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::resolve_type_vars_in_body
  47: rustc::ty::context::tls::enter
  48: rustc::infer::InferCtxtBuilder::enter
  49: rustc_typeck::check::typeck_tables_of
  50: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::compute_result
  51: rustc::dep_graph::graph::DepGraph::with_task_impl
  52: rustc_errors::Handler::track_diagnostics
  53: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  54: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::force
  55: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get
  56: rustc::ty::maps::TyCtxtAt::typeck_tables_of
  57: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure
  58: rustc::session::Session::track_errors
  59: rustc_typeck::check::typeck_item_bodies
  60: rustc::dep_graph::graph::DepGraph::with_task_impl
  61: rustc_errors::Handler::track_diagnostics
  62: rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::cycle_check
  63: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::force
  64: rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get
  65: rustc::ty::maps::TyCtxtAt::typeck_item_bodies
  66: rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies
  67: rustc_typeck::check_crate
  68: <std::thread::local::LocalKey<T>>::with
  69: <std::thread::local::LocalKey<T>>::with
  70: rustc::ty::context::TyCtxt::create_and_enter
  71: rustc_driver::driver::compile_input
  72: rustc_driver::run_compiler_impl
  73: syntax::with_globals
  74: rustc_driver::run
  75: rustc_driver::main
  76: std::rt::lang_start::{{closure}}
  77: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:306
  78: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  79: std::rt::lang_start_internal
             at libstd/panicking.rs:285
             at libstd/panic.rs:361
             at libstd/rt.rs:58
  80: main
  81: __libc_start_main
  82: <unknown>
