
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 18446744073709551615', /rustc/b25ee644971a168287ee166edbd11642dbcfeab8/src/libcore/slice/mod.rs:2700:14
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:212
   6: rustc::util::common::panic_hook
   7: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:479
   8: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:382
   9: rust_begin_unwind
             at src/libstd/panicking.rs:309
  10: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  11: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:61
  12: rustc_typeck::check::check_item_type
  13: rustc::hir::map::Map::visit_item_likes_in_module
  14: rustc_typeck::check::check_mod_item_types
  15: rustc::ty::query::__query_compute::check_mod_item_types
  16: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_mod_item_types>::compute
  17: rustc::dep_graph::graph::DepGraph::with_task_impl
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  19: rustc_typeck::check_crate::{{closure}}
  20: rustc::util::common::time
  21: rustc_typeck::check_crate
  22: rustc_interface::passes::analysis
  23: rustc::ty::query::__query_compute::analysis
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  26: rustc::ty::context::tls::enter_global
  27: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  28: rustc_interface::passes::create_global_ctxt::{{closure}}
  29: rustc_interface::interface::run_compiler_in_existing_thread_pool
  30: std::thread::local::LocalKey<T>::with
  31: scoped_tls::ScopedKey<T>::set
  32: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
#0 [check_mod_item_types] checking item types in top-level module
#1 [analysis] running analysis passes on this crate
end of query stack
