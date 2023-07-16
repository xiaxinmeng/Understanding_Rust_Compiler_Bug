
% RUST_BACKTRACE=1 rustc +nightly --crate-type=lib -Ztreat-err-as-bug=1 issue.rs
error: internal compiler error: broken MIR in DefId(0:16 ~ issue[8787]::test) (Terminator { source_info: SourceInfo { span: issue.rs:13:5: 13:25 (#0), scope: scope[0] }, kind: _1 = assert_is_func::<<Foo<'_> as Bar>::Type>(move _2) -> [return: bb1, unwind: bb2] }): bad arg #0 (fn(()) <- for<'r> fn(<Foo<'r> as Bar>::Type)): NoSolution
  --> issue.rs:13:20
   |
13 |     assert_is_func(func);
   |                    ^^^^

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:990:27
stack backtrace:
   0: std::panicking::begin_panic
   1: rustc_errors::HandlerInner::emit_diagnostic
   2: rustc_errors::HandlerInner::emit_diag_at_span
   3: rustc_errors::HandlerInner::span_bug
   4: rustc_errors::Handler::delay_span_bug
   5: rustc_mir::borrow_check::type_check::TypeChecker::typeck_mir
   6: rustc_mir::borrow_check::type_check::type_check
   7: rustc_mir::borrow_check::nll::compute_regions
   8: rustc_mir::borrow_check::do_mir_borrowck
   9: rustc_infer::infer::InferCtxtBuilder::enter
  10: rustc_mir::borrow_check::mir_borrowck
  11: core::ops::function::FnOnce::call_once
  12: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute
  13: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  14: rustc_data_structures::stack::ensure_sufficient_stack
  15: rustc_query_system::query::plumbing::get_query_impl
  16: rustc_query_system::query::plumbing::ensure_query_impl
  17: rustc_session::utils::<impl rustc_session::session::Session>::time
  18: rustc_interface::passes::analysis
  19: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  20: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  21: rustc_data_structures::stack::ensure_sufficient_stack
  22: rustc_query_system::query::plumbing::get_query_impl
  23: rustc_interface::passes::QueryContext::enter
  24: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  25: rustc_span::with_source_map
  26: rustc_interface::interface::create_compiler_and_run
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (158f8d034 2020-12-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug=1 --crate-type lib

query stack during panic:
#0 [mir_borrowck] borrow-checking `test`
#1 [analysis] running analysis passes on this crate
end of query stack
