
thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: &str }
- dep-node: layout_raw(ad738982f22c678a-9e7e0f2c32792904)', /rustc/a435b49e86d16e98dcc6595dd471f95e823f41aa/compiler/rustc_query_system/src/query/plumbing.rs:645:5
stack backtrace:
   0: _rust_begin_unwind
   1: std::panicking::begin_panic_fmt
   2: rustc_query_system::query::plumbing::get_query_impl
   3: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_raw
   4: <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of
   5: <rustc_lint::context::LateContext as rustc_target::abi::LayoutOf>::layout_of
   6: <clippy_lints::zero_sized_map_values::ZeroSizedMapValues as rustc_lint::passes::LateLintPass>::check_ty
   7: <rustc_lint::late::LateLintPassObjects as rustc_lint::passes::LateLintPass>::check_ty
   8: rustc_hir::intravisit::walk_ty
   9: rustc_hir::intravisit::walk_fn
  10: <rustc_lint::late::LateContextAndPass<T> as rustc_hir::intravisit::Visitor>::visit_fn
  11: rustc_hir::intravisit::walk_item
  12: rustc_hir::intravisit::Visitor::visit_nested_item
  13: rustc_hir::intravisit::walk_crate
  14: rustc_lint::late::late_lint_pass_crate
  15: rustc_lint::late::late_lint_crate
  16: rustc_data_structures::sync::join
  17: rustc_interface::passes::analysis::{{closure}}::{{closure}}
  18: rustc_session::utils::<impl rustc_session::session::Session>::time
  19: rustc_interface::passes::analysis
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  21: rustc_data_structures::stack::ensure_sufficient_stack
  22: rustc_query_system::query::plumbing::force_query_with_job
  23: rustc_query_system::query::plumbing::get_query_impl
  24: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  25: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  26: rustc_span::with_source_map
  27: rustc_interface::interface::create_compiler_and_run
  28: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.54 (a435b49e 2021-06-28)

query stack during panic:
#0 [analysis] running analysis passes on this crate
end of query stack
