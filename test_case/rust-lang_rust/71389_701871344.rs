
$ RUST_BACKTRACE=1 rustc +devel fil7.rs -Ztreat-err-as-bug
error[E0277]: the trait bound `&mut i32: FromStr` is not satisfied
 --> fil7.rs:2:14
  |
2 |     n = "42".parse().unwrap();
  |              ^^^^^ the trait `FromStr` is not implemented for `&mut i32`
  |
  = help: the following implementations were found:
            <i32 as FromStr>

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:977:27
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::emit_diagnostic
   2: rustc_errors::Handler::emit_diagnostic
   3: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
   4: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error
   5: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error
   6: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
   7: rustc_typeck::check::fn_ctxt::FnCtxt::select_obligations_where_possible
   8: rustc_infer::infer::InferCtxtBuilder::enter
   9: rustc_typeck::check::inherited::InheritedBuilder::enter
  10: rustc_typeck::check::typeck_with_fallback
  11: rustc_typeck::check::typeck
  12: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck>::compute
  13: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  14: rustc_data_structures::stack::ensure_sufficient_stack
  15: rustc_query_system::query::plumbing::get_query_impl
  16: rustc_query_system::query::plumbing::ensure_query_impl
  17: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::par_body_owners
  18: rustc_typeck::check::typeck_item_bodies
  19: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_item_bodies>::compute
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  21: rustc_data_structures::stack::ensure_sufficient_stack
  22: rustc_query_system::query::plumbing::get_query_impl
  23: rustc_session::utils::<impl rustc_session::session::Session>::time
  24: rustc_typeck::check_crate
  25: rustc_interface::passes::analysis
  26: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  27: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  28: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
  29: rustc_data_structures::stack::ensure_sufficient_stack
  30: rustc_query_system::query::plumbing::get_query_impl
  31: rustc_interface::passes::QueryContext::enter
  32: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  33: rustc_span::with_source_map
  34: rustc_interface::interface::create_compiler_and_run
  35: scoped_tls::ScopedKey<T>::set
  36: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z treat-err-as-bug

query stack during panic:
#0 [typeck] type-checking `t`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
