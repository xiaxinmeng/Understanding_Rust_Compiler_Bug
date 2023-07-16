
error: internal compiler error: src/librustc/dep_graph/graph.rs:680: DepNode Hir(manifold[2147]::manifold[0]::Vertex[0]::position[0]) should have been pre-allocated but wasn't.

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:931:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:77
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:188
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:205
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:468
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_opt
  19: rustc::util::bug::opt_span_bug_fmt
  20: rustc::util::bug::bug_fmt
  21: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  22: rustc::dep_graph::graph::DepGraph::try_mark_green
  23: rustc::dep_graph::graph::DepGraph::try_mark_green_and_read
  24: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  25: <rustc_typeck::outlives::implicit_infer::InferVisitor as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item
  26: rustc::hir::Crate::visit_all_item_likes
  27: rustc_typeck::outlives::inferred_outlives_crate
  28: rustc::ty::query::__query_compute::inferred_outlives_crate
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  31: rustc_typeck::outlives::inferred_outlives_of
  32: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::inferred_outlives_of>::compute
  33: rustc::dep_graph::graph::DepGraph::with_task_impl
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query
  35: rustc::ty::query::plumbing::force_from_dep_node
  36: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  37: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  38: rustc::dep_graph::graph::DepGraph::try_mark_green
  39: rustc::dep_graph::graph::DepGraph::try_mark_green_and_read
  40: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  41: <rustc_typeck::collect::CollectItemTypesVisitor as rustc::hir::intravisit::Visitor>::visit_item
  42: rustc::hir::map::Map::visit_item_likes_in_module
  43: rustc_typeck::collect::collect_mod_item_types
  44: rustc::ty::query::__query_compute::collect_mod_item_types
  45: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::collect_mod_item_types>::compute
  46: rustc::dep_graph::graph::DepGraph::with_task_impl
  47: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  48: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  49: rustc_typeck::check_crate::{{closure}}::{{closure}}
  50: rustc::util::common::time
  51: rustc_typeck::check_crate
  52: rustc_interface::passes::analysis
  53: rustc::ty::query::__query_compute::analysis
  54: rustc::dep_graph::graph::DepGraph::with_task_impl
  55: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  56: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  57: rustc_interface::passes::create_global_ctxt::{{closure}}
  58: rustc_interface::passes::BoxedGlobalCtxt::enter
  59: rustc_interface::interface::run_compiler_in_existing_thread_pool
  60: std::thread::local::LocalKey<T>::with
  61: scoped_tls::ScopedKey<T>::set
  62: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0 (73528e339 2019-12-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_of] processing `manifold::Vertex::position`
#1 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
#2 [inferred_outlives_of] processing `manifold::EdgeHandle`
#3 [predicates_of] processing `manifold::EdgeHandle`
#4 [collect_mod_item_types] collecting item types in module `manifold`
#5 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
