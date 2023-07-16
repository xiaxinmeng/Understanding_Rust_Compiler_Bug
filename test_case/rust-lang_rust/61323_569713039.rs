
error: internal compiler error: src/librustc/dep_graph/graph.rs:714: try_mark_previous_green() - Forcing the DepNode should have set its color

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:931:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::begin_panic
   8: rustc_errors::HandlerInner::bug
   9: rustc_errors::Handler::bug
  10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc::ty::context::tls::with_opt::{{closure}}
  12: rustc::ty::context::tls::with_context_opt
  13: rustc::ty::context::tls::with_opt
  14: rustc::util::bug::opt_span_bug_fmt
  15: rustc::util::bug::bug_fmt
  16: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  17: rustc::dep_graph::graph::DepGraph::try_mark_green
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  19: rustc::ty::AdtDef::sized_constraint_for_ty
  20: <core::iter::adapters::flatten::FlatMap<I,U,F> as core::iter::traits::iterator::Iterator>::next
  21: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  22: <I as rustc::ty::context::InternAs<[T],R>>::intern_with
  23: rustc::ty::adt_sized_constraint
  24: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::adt_sized_constraint>::compute
  25: rustc::dep_graph::graph::DepGraph::with_task_impl
  26: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query
  27: rustc::ty::query::plumbing::force_from_dep_node
  28: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  29: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  30: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  31: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  32: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  33: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  34: rustc::dep_graph::graph::DepGraph::try_mark_green
  35: rustc::dep_graph::graph::DepGraph::try_mark_green_and_read
  36: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  37: __rust_maybe_catch_panic
  38: rustc_data_structures::sync::par_for_each_in
  39: __rust_maybe_catch_panic
  40: rustc::hir::Crate::par_visit_all_item_likes
  41: rustc::util::common::time
  42: rustc_typeck::check_crate
  43: rustc_interface::passes::analysis
  44: rustc::ty::query::__query_compute::analysis
  45: rustc::dep_graph::graph::DepGraph::with_task_impl
  46: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  47: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  48: rustc_interface::passes::create_global_ctxt::{{closure}}
  49: rustc_interface::passes::BoxedGlobalCtxt::enter
  50: rustc_interface::interface::run_compiler_in_existing_thread_pool
  51: std::thread::local::LocalKey<T>::with
  52: scoped_tls::ScopedKey<T>::set
  53: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0 (73528e339 2019-12-16) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [adt_sized_constraint] processing `ast::comparison::Comparison`
#1 [adt_sized_constraint] processing `ast::expression::Expression`
#2 [analysis] running analysis passes on this crate
end of query stack
