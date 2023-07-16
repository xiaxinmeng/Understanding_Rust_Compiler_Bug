
error: internal compiler error: src/librustc/ty/query/plumbing.rs:1195: Cannot force dep node: coherent_trait(core[30a9]::ops[0]::drop[0]::Drop[0])

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:635:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc::ty::query::plumbing::force_from_dep_node
  15: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  16: rustc::dep_graph::graph::DepGraph::try_mark_green
  17: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  18: rustc_typeck::check::check_item_type
  19: rustc::hir::map::Map::visit_item_likes_in_module
  20: rustc_typeck::check::check_mod_item_types
  21: rustc::ty::query::__query_compute::check_mod_item_types
  22: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_mod_item_types>::compute
  23: rustc::dep_graph::graph::DepGraph::with_task_impl
  24: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  25: rustc::session::Session::track_errors
  26: rustc::util::common::time
  27: rustc_typeck::check_crate
  28: rustc_interface::passes::analysis
  29: rustc::ty::query::__query_compute::analysis
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  33: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  34: rustc_interface::passes::create_global_ctxt::{{closure}}
  35: rustc_interface::interface::run_compiler_in_existing_thread_pool
  36: std::thread::local::LocalKey<T>::with
  37: scoped_tls::ScopedKey<T>::set
  38: syntax::with_globals
query stack during panic:
#0 [adt_destructor] processing `ServerFacade`
#1 [check_mod_item_types] checking item types in top-level module
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
