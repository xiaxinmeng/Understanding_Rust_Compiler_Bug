
    Checking tikv v3.0.0-beta (/home/ubuntu/tikv)
error: internal compiler error: src/librustc_typeck/collect.rs:1311: unexpected non-type NodeGenericParam

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:589:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
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
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc_typeck::collect::type_of
  15: rustc::ty::query::__query_compute::type_of
  16: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::type_of<'tcx>>::compute
  17: rustc::dep_graph::graph::DepGraph::with_task_impl
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_with
  19: clippy_lints::types::check_ty
  20: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_struct_field
  21: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_variant_data
  22: rustc::hir::intravisit::walk_item
  23: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  24: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  25: rustc::hir::intravisit::walk_item
  26: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  27: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  28: rustc::hir::intravisit::walk_item
  29: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  30: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  31: rustc::hir::intravisit::walk_item
  32: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  33: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  34: rustc::hir::intravisit::walk_item
  35: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  36: <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod
  37: rustc::hir::intravisit::walk_crate
  38: rustc::lint::context::check_crate
  39: rustc::util::common::time
  40: <std::thread::local::LocalKey<T>>::with
  41: rustc::ty::context::TyCtxt::create_and_enter
  42: rustc_driver::driver::compile_input
  43: <scoped_tls::ScopedKey<T>>::set
  44: rustc_driver::run_compiler
  45: <scoped_tls::ScopedKey<T>>::set
query stack during panic:
#0 [type_of] processing `raftstore::store::fsm::batch::Batch::N`
end of query stack
