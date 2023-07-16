
  19: rustc::util::bug::bug_fmt
  20: rustc_typeck::collect::fn_sig
  21: rustc::ty::query::__query_compute::fn_sig
  22: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::fn_sig>::compute
  23: rustc::dep_graph::graph::DepGraph::with_task_impl
  24: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  25: rustc::ty::constness::provide::is_const_fn_raw
  26: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::is_const_fn_raw>::compute::{{closure}}
             at /rustc/7fa046534e944193cc47b795b9396a7fcf411d9f/src/librustc/ty/query/plumbing.rs:984
  27: rustc::ty::query::__query_compute::is_const_fn_raw
             at /rustc/7fa046534e944193cc47b795b9396a7fcf411d9f/src/librustc/ty/query/plumbing.rs:935
