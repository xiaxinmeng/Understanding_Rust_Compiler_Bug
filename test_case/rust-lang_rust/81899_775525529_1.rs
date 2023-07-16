
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_mir/src/borrow_check/region_infer/mod.rs:2136:35
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9778068cbc1e06cc3685422323ff38a2f397de26/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /rustc/9778068cbc1e06cc3685422323ff38a2f397de26/library/core/src/panicking.rs:92:14
   2: core::panicking::panic
             at /rustc/9778068cbc1e06cc3685422323ff38a2f397de26/library/core/src/panicking.rs:50:5
   3: rustc_mir::borrow_check::region_infer::RegionInferenceContext::find_outlives_blame_span
   4: rustc_mir::borrow_check::do_mir_borrowck
   5: rustc_infer::infer::InferCtxtBuilder::enter
   6: rustc_mir::borrow_check::mir_borrowck
   7: core::ops::function::FnOnce::call_once
   8: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute
   9: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  10: rustc_data_structures::stack::ensure_sufficient_stack
  11: rustc_query_system::query::plumbing::force_query_with_job
  12: rustc_query_system::query::plumbing::get_query_impl
  13: rustc_query_system::query::plumbing::ensure_query_impl
  14: rustc_session::utils::<impl rustc_session::session::Session>::time
  15: rustc_interface::passes::analysis
  16: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  17: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  18: rustc_data_structures::stack::ensure_sufficient_stack
  19: rustc_query_system::query::plumbing::force_query_with_job
  20: rustc_query_system::query::plumbing::get_query_impl
  21: rustc_interface::passes::QueryContext::enter
  22: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  23: rustc_span::with_source_map
  24: rustc_interface::interface::create_compiler_and_run
  25: rustc_span::with_session_globals
