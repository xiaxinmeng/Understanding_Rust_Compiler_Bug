
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: src/librustc_mir/borrow_check/universal_regions.rs:762: cannot convert `ReEarlyBound(0, 'a)` to a region vid

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:905:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:84
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1025
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:193
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:475
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_opt
  18: rustc::util::bug::opt_span_bug_fmt
  19: rustc::util::bug::bug_fmt
  20: rustc_mir::borrow_check::universal_regions::UniversalRegionIndices::to_region_vid::{{closure}}
  21: rustc_mir::borrow_check::universal_regions::UniversalRegionIndices::to_region_vid
  22: rustc_mir::borrow_check::type_check::constraint_conversion::ConstraintConversion::convert_all
  23: rustc_mir::borrow_check::type_check::TypeChecker::fully_perform_op
  24: rustc_mir::borrow_check::type_check::type_check
  25: rustc_mir::borrow_check::nll::compute_regions
  26: rustc_mir::borrow_check::do_mir_borrowck
  27: rustc::ty::context::GlobalCtxt::enter_local
  28: rustc_mir::borrow_check::mir_borrowck
  29: rustc::ty::query::__query_compute::mir_borrowck
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  34: rustc_mir::transform::optimized_mir
  35: rustc::ty::query::__query_compute::optimized_mir
  36: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  37: rustc::dep_graph::graph::DepGraph::with_task_impl
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  39: rustc_mir::interpret::eval_context::InterpCx<M>::load_mir
  40: rustc_mir::const_eval::const_eval_raw_provider
  41: rustc::ty::query::__query_compute::const_eval_raw
  42: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  43: rustc::dep_graph::graph::DepGraph::with_task_impl
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  45: rustc_mir::const_eval::const_eval_provider
  46: rustc::ty::query::__query_compute::const_eval
  47: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  48: rustc::dep_graph::graph::DepGraph::with_task_impl
  49: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  50: rustc_mir::const_eval::const_eval_provider
  51: rustc::ty::query::__query_compute::const_eval
  52: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  53: rustc::dep_graph::graph::DepGraph::with_task_impl
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  55: rustc::ty::sty::Const::eval::{{closure}}
  56: rustc::ty::sty::Const::eval
  57: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  58: <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  59: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  60: <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  61: rustc::traits::project::normalize
  62: rustc_typeck::check::FnCtxt::normalize_associated_types_in
  63: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
  64: rustc::ty::context::GlobalCtxt::enter_local
  65: rustc_typeck::check::wfcheck::check_item_well_formed
  66: rustc::ty::query::__query_compute::check_item_well_formed
  67: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_item_well_formed>::compute
  68: rustc::dep_graph::graph::DepGraph::with_task_impl
  69: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  70: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  71: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:78
  72: rustc_data_structures::sync::par_for_each_in
  73: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:78
  74: rustc::hir::Crate::par_visit_all_item_likes
  75: rustc::util::common::time
  76: rustc_typeck::check_crate
  77: rustc_interface::passes::analysis
  78: rustc::ty::query::__query_compute::analysis
  79: rustc::dep_graph::graph::DepGraph::with_task_impl
  80: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  81: rustc::ty::context::tls::enter_global
  82: rustc_interface::interface::run_compiler_in_existing_thread_pool
  83: std::thread::local::LocalKey<T>::with
  84: scoped_tls::ScopedKey<T>::set
  85: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0 (5e1a79984 2020-01-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [mir_borrowck] processing `DataWrapper::data::{{constant}}#0`
#1 [optimized_mir] processing `DataWrapper::data::{{constant}}#0`
#2 [const_eval_raw] const-evaluating `DataWrapper::data::{{constant}}#0`
#3 [const_eval] const-evaluating + checking `DataWrapper::data::{{constant}}#0`
#4 [const_eval] const-evaluating + checking `DataWrapper::data::{{constant}}#0`
#5 [check_item_well_formed] processing `DataWrapper`
#6 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error

error: could not compile `playground`.

To learn more, run the command again with --verbose.
