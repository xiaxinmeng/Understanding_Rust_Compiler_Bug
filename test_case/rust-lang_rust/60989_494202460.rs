
error: internal compiler error: src/librustc_typeck/check/mod.rs:2456: no type for node 48: type B (hir_id=HirId { owner: DefIndex(16), local_id: 13 }) in fcx 0x7f82da7ec990

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:637:9
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
  14: rustc_typeck::check::FnCtxt::node_ty
  15: <rustc_typeck::check::writeback::WritebackCx as rustc::hir::intravisit::Visitor>::visit_ty
  16: rustc::hir::intravisit::walk_path
  17: rustc::hir::intravisit::walk_ty
  18: <rustc_typeck::check::writeback::WritebackCx as rustc::hir::intravisit::Visitor>::visit_ty
  19: rustc::hir::intravisit::walk_path
  20: <rustc_typeck::check::writeback::WritebackCx as rustc::hir::intravisit::Visitor>::visit_expr
  21: rustc::hir::intravisit::walk_expr
  22: <rustc_typeck::check::writeback::WritebackCx as rustc::hir::intravisit::Visitor>::visit_expr
  23: <rustc_typeck::check::writeback::WritebackCx as rustc::hir::intravisit::Visitor>::visit_local
  24: rustc::hir::intravisit::walk_expr
  25: <rustc_typeck::check::writeback::WritebackCx as rustc::hir::intravisit::Visitor>::visit_expr
  26: rustc_typeck::check::writeback::<impl rustc_typeck::check::FnCtxt>::resolve_type_vars_in_body
  27: rustc::ty::context::GlobalCtxt::enter_local
  28: rustc_typeck::check::typeck_tables_of
  29: rustc::ty::query::__query_compute::typeck_tables_of
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  33: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  34: rustc_typeck::check::typeck_item_bodies
  35: rustc::ty::query::__query_compute::typeck_item_bodies
  36: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_item_bodies>::compute
  37: rustc::dep_graph::graph::DepGraph::with_task_impl
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  39: rustc::util::common::time
  40: rustc_typeck::check_crate
  41: rustc_interface::passes::analysis
  42: rustc::ty::query::__query_compute::analysis
  43: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  44: rustc::dep_graph::graph::DepGraph::with_task_impl
  45: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  46: rustc::ty::context::tls::enter_global
  47: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  48: rustc_interface::passes::create_global_ctxt::{{closure}}
  49: rustc_interface::interface::run_compiler_in_existing_thread_pool
  50: std::thread::local::LocalKey<T>::with
  51: scoped_tls::ScopedKey<T>::set
  52: syntax::with_globals
query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0-nightly (6afcb5628 2019-05-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.
