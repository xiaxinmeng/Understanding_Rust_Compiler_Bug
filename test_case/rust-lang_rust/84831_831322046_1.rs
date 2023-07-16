
error[E0423]: expected value, found crate `std`
 --> src/main.rs:2:5
  |
2 |     std::<0>; 
  |     ^^^^^^^^ not a value

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_metadata/rmeta/decoder.rs:854:9
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys_common::backtrace::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: rustc_driver::report_ice
   7: std::panicking::rust_panic_with_hook
   8: rust_begin_unwind
   9: core::panicking::panic_fmt
  10: core::panicking::panic
  11: rustc_metadata::rmeta::decoder::<impl rustc_metadata::creader::CrateMetadataRef>::get_generics
  12: rustc_metadata::rmeta::decoder::cstore_impl::provide_extern::generics_of
  13: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::generics_of>::compute
  14: rustc::dep_graph::graph::DepGraph::with_task_impl
  15: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  16: rustc_typeck::collect::type_of::type_of
  17: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::type_of>::compute
  18: rustc::dep_graph::graph::DepGraph::with_task_impl
  19: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  20: rustc::ty::context::GlobalCtxt::enter_local
  21: rustc_infer::infer::InferCtxtBuilder::enter
  22: rustc_typeck::check::typeck_tables_of_with_fallback
  23: rustc_typeck::check::typeck_tables_of
  24: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  25: rustc::dep_graph::graph::DepGraph::with_task_impl
  26: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  27: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  28: rustc_typeck::check::typeck_item_bodies
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_item_bodies>::compute
  30: rustc::dep_graph::graph::DepGraph::with_task_impl
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  32: rustc_session::utils::<impl rustc_session::session::Session>::time
  33: rustc_typeck::check_crate
  34: rustc_interface::passes::analysis
  35: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  36: rustc::dep_graph::graph::DepGraph::with_eval_always_task
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  38: rustc::ty::context::tls::enter_global
  39: rustc_interface::interface::run_compiler_in_existing_thread_pool
  40: scoped_tls::ScopedKey<T>::set
  41: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0 (0a17c4c19 2020-03-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [generics_of] processing `std`
#1 [type_of] processing `f::{{constant}}#0`
#2 [typeck_tables_of] type-checking `f::{{constant}}#0`
#3 [typeck_item_bodies] type-checking all item bodies
#4 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
