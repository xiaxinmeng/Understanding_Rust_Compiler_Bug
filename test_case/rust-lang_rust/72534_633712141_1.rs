
export RUST_BACKTRACE=1; cargo +stage1 build
   Compiling binding2 v0.1.0 (/Users/chris/Desktop/tests/rustlang-tests/binding2)
thread 'rustc' panicked at 'assertion failed: should_monomorphize_locally(tcx, &instance)', src/librustc_mir/monomorphize/collector.rs:366:13
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys_common::backtrace::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: rustc_driver::report_ice
   7: std::panicking::rust_panic_with_hook
   8: std::panicking::begin_panic
   9: rustc_mir::monomorphize::collector::collect_items_rec
  10: rustc_session::utils::<impl rustc_session::session::Session>::time
  11: rustc_mir::monomorphize::collector::collect_crate_mono_items
  12: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  13: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_and_partition_mono_items>::compute
  14: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  15: rustc_data_structures::stack::ensure_sufficient_stack
  16: rustc_query_system::query::plumbing::get_query_impl
  17: rustc_codegen_ssa::base::codegen_crate
  18: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  19: rustc_session::utils::<impl rustc_session::session::Session>::time
  20: rustc_middle::ty::context::tls::enter_global
  21: rustc_interface::queries::Query<T>::compute
  22: rustc_interface::queries::Queries::ongoing_codegen
  23: rustc_interface::interface::run_compiler_in_existing_thread_pool
  24: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.45.0-dev running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
