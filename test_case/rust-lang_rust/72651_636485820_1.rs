
error: internal compiler error: src/librustc_mir/transform/generator.rs:713: Broken MIR: generator contains type &StructB in MIR, but typeck only knows about {std::future::ResumeTy, StructB, std::option::Option<StructB>, impl std::future::Future, ()}
  --> src/main.rs:20:16
   |
20 |   async fn ice() {
   |  ________________^
21 | |     match Some(StructB {}) {
22 | |         Some(struct_b) if get_struct_a_async().await.fn_taking_struct_b(&struct_b) => {}
23 | |         _ => {}
24 | |     }
25 | | }
   | |_^

thread 'rustc' panicked at 'Box<Any>', /rustc/74e80468347471779be6060d8d7d6d04e98e467f/src/libstd/macros.rs:13:23
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1076
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:490
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::span_bug
  14: rustc_errors::Handler::span_bug
  15: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc_middle::ty::context::tls::with_opt::{{closure}}
  17: rustc_middle::ty::context::tls::with_opt
  18: rustc_middle::util::bug::opt_span_bug_fmt
  19: rustc_middle::util::bug::span_bug_fmt
  20: <rustc_mir::transform::generator::StateTransform as rustc_mir::transform::MirPass>::run_pass
  21: rustc_mir::transform::run_passes
  22: rustc_mir::transform::run_optimization_passes
  23: rustc_mir::transform::optimized_mir
  24: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::optimized_mir>::compute
  25: rustc_middle::dep_graph::<impl rustc_query_system::dep_graph::DepKind for rustc_middle::dep_graph::dep_node::DepKind>::with_deps
  26: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  27: rustc_query_system::query::plumbing::get_query_impl::{{closure}}
  28: rustc_query_system::query::plumbing::get_query_impl
  29: rustc_mir::util::pretty::write_mir_pretty
  30: rustc_mir::transform::dump_mir::emit_mir
  31: rustc_interface::passes::start_codegen
  32: rustc_middle::ty::context::tls::enter_global
  33: rustc_interface::queries::Queries::ongoing_codegen
  34: rustc_interface::interface::run_compiler_in_existing_thread_pool
  35: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.45.0-nightly (74e804683 2020-05-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] processing `ice::{{closure}}#0`
end of query stack
