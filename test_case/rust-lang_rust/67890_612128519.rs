
error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:606: collection encountered polymorphic constant
  --> /home/mateusz/.cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.25/src/int/mod.rs:50:5
   |
50 |     const ONE: Self;
   |     ^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'Box<Any>', <::std::macros::panic macros>:2:4
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::panicking::default_hook::{{closure}}
   4: std::panicking::default_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::begin_panic
   8: rustc_errors::HandlerInner::span_bug
   9: rustc_errors::Handler::span_bug
  10: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
  11: rustc_middle::ty::context::tls::with_opt::{{closure}}
  12: rustc_middle::ty::context::tls::with_opt
  13: rustc_middle::util::bug::opt_span_bug_fmt
  14: rustc_middle::util::bug::span_bug_fmt
  15: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_const
  16: <rustc_mir::monomorphize::collector::MirNeighborCollector as rustc_middle::mir::visit::Visitor>::visit_rvalue
  17: rustc_mir::monomorphize::collector::collect_items_rec
  18: rustc_mir::monomorphize::collector::collect_items_rec
  19: rustc_session::utils::<impl rustc_session::session::Session>::time
  20: rustc_mir::monomorphize::collector::collect_crate_mono_items
  21: rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items
  22: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_and_partition_mono_items>::compute
  23: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  24: rustc_query_system::query::plumbing::get_query
  25: rustc_codegen_ssa::base::codegen_crate
  26: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  27: rustc_interface::passes::start_codegen
  28: rustc_middle::ty::context::tls::enter_global
  29: rustc_interface::queries::Queries::ongoing_codegen
  30: rustc_interface::interface::run_compiler_in_existing_thread_pool
  31: scoped_tls::ScopedKey<T>::set
  32: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C panic=abort -C debug-assertions=no --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error

error: could not compile `compiler_builtins`.
