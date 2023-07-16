
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:70
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:58
             at src/libstd/panicking.rs:200
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:215
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:482
   6: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:385
   7: rust_begin_unwind
             at src/libstd/panicking.rs:312
   8: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
   9: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:61
  10: <ena::unify::UnificationTable<S>>::get_root_key
  11: rustc::infer::InferCtxt::shallow_resolve
  12: <rustc::infer::resolve::OpportunisticTypeResolver<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  13: rustc::infer::InferCtxt::probe
  14: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  15: rustc_typeck::check::method::probe::ProbeContext::pick_method
  16: rustc_typeck::check::method::probe::ProbeContext::pick_core
  17: <core::iter::FilterMap<I, F> as core::iter::iterator::Iterator>::next
  18: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  19: rustc::infer::InferCtxt::probe
  20: rustc_typeck::check::method::probe::ProbeContext::pick
  21: rustc::infer::InferCtxt::probe
  22: rustc_typeck::check::method::probe::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::probe_op
  23: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
  24: rustc_typeck::check::method::probe::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::probe_for_return_type
  25: rustc_typeck::check::FnCtxt::suggest_ref_or_into
  26: rustc_typeck::check::FnCtxt::suggest_mismatched_types_on_tail
  27: <rustc_typeck::check::coercion::CoerceMany<'gcx, 'tcx, 'exprs, E>>::coerce_inner
  28: rustc_typeck::check::FnCtxt::check_block_with_expected
  29: rustc_typeck::check::FnCtxt::check_expr_kind
  30: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  31: rustc_typeck::check::FnCtxt::check_return_expr
  32: rustc_typeck::check::check_fn
  33: rustc::ty::context::GlobalCtxt::enter_local
  34: rustc_typeck::check::typeck_tables_of
  35: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query
  39: rustc::session::Session::track_errors
  40: rustc_typeck::check::typeck_item_bodies
  41: rustc::ty::query::__query_compute::typeck_item_bodies
  42: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute
  43: rustc::dep_graph::graph::DepGraph::with_task_impl
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  45: rustc::util::common::time
  46: rustc_typeck::check_crate
  47: <std::thread::local::LocalKey<T>>::with
  48: rustc::ty::context::TyCtxt::create_and_enter
  49: rustc_driver::driver::compile_input
  50: rustc_driver::run_compiler_with_pool
  51: <scoped_tls::ScopedKey<T>>::set
  52: rustc_driver::run_compiler
  53: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [typeck_tables_of] processing `ice`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
