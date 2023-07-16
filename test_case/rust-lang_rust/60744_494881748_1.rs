
error: internal compiler error: src/librustc/traits/codegen/mod.rs:160: Uninferred types/regions in `VtableImplData(impl_def_id=DefId(0:16 ~ fixpt[712f]::{{impl}}[0]), substs=[Const { ty: u8, val: Infer(Var(_#0c)) }], nested=[])`

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
  14: rustc::traits::codegen::<impl rustc::infer::InferCtxt>::drain_fulfillment_cx_or_panic
  15: rustc::ty::context::GlobalCtxt::enter_local
  16: rustc::traits::codegen::codegen_fulfill_obligation
  17: rustc::ty::query::__query_compute::codegen_fulfill_obligation
  18: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::codegen_fulfill_obligation>::compute
  19: rustc::dep_graph::graph::DepGraph::with_task_impl
  20: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  21: rustc::ty::instance::Instance::resolve
  22: rustc_mir::lints::check
  23: rustc::ty::context::GlobalCtxt::enter_local
  24: rustc_mir::build::mir_build
  25: rustc_mir::transform::mir_built
  26: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_built>::compute
  27: rustc::dep_graph::graph::DepGraph::with_task_impl
  28: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query
  29: rustc::ty::query::plumbing::force_from_dep_node
  30: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  31: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  32: rustc::dep_graph::graph::DepGraph::try_mark_green
  33: rustc::dep_graph::graph::DepGraph::try_mark_green_and_read
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  35: rustc_mir::transform::mir_validated
  36: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_validated>::compute
  37: rustc::dep_graph::graph::DepGraph::with_task_impl
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::force_query
  39: rustc::ty::query::plumbing::force_from_dep_node
  40: rustc::dep_graph::graph::DepGraph::try_mark_previous_green
  41: rustc::dep_graph::graph::DepGraph::try_mark_green
  42: rustc::dep_graph::graph::DepGraph::try_mark_green_and_read
  43: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  44: rustc::util::common::time
  45: rustc_interface::passes::analysis
  46: rustc::ty::query::__query_compute::analysis
  47: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  48: rustc::dep_graph::graph::DepGraph::with_task_impl
  49: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  50: rustc::ty::context::tls::enter_global
  51: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  52: rustc_interface::passes::create_global_ctxt::{{closure}}
  53: rustc_interface::interface::run_compiler_in_existing_thread_pool
  54: std::thread::local::LocalKey<T>::with
  55: scoped_tls::ScopedKey<T>::set
  56: syntax::with_globals
query stack during panic:
#0 [codegen_fulfill_obligation] checking if `std::clone::Clone` fulfills its obligations
#1 [mir_built] processing `test_foo`
#2 [mir_const] processing `test_foo`
#3 [mir_validated] processing `test_foo`
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
