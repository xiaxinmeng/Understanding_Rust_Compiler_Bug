
stack backtrace:
   0: rust_begin_unwind
             at /rustc/45b3c28518e4c45dfd12bc2c4400c0d0e9639927/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /rustc/45b3c28518e4c45dfd12bc2c4400c0d0e9639927/library/core/src/panicking.rs:92:14
   2: core::panicking::panic
             at /rustc/45b3c28518e4c45dfd12bc2c4400c0d0e9639927/library/core/src/panicking.rs:50:5
   3: rustc_middle::hir::map::Map::body_owner
   4: rustc_middle::hir::map::Map::body_owner_def_id
   5: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
   6: rustc_typeck::check::typeck_item_bodies
   7: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
   8: rustc_data_structures::stack::ensure_sufficient_stack
   9: rustc_query_system::query::plumbing::force_query_with_job
  10: rustc_query_system::query::plumbing::get_query_impl
  11: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::typeck_item_bodies
  12: rustc_session::utils::<impl rustc_session::session::Session>::time
  13: rustc_typeck::check_crate
  14: rustc_interface::passes::analysis
  15: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  16: rustc_data_structures::stack::ensure_sufficient_stack
  17: rustc_query_system::query::plumbing::force_query_with_job
  18: rustc_query_system::query::plumbing::get_query_impl
  19: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
  20: rustc_interface::passes::QueryContext::enter
  21: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  22: rustc_span::with_source_map
  23: rustc_interface::interface::create_compiler_and_run
  24: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
